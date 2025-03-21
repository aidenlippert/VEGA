use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use serde_json::Value;
use tch::{Tensor, nn::{self, Module}};
use ssi::vc::{Credential, Contexts, Issuer, OneOrMany, URI, VCDateTime, Context, StringOrURI};
use ssi::ldp::ProofSuiteType;
use ssi::did::VerificationRelationship;
use chrono::{DateTime, Utc};
use hex;

pub struct VegaStar {
    did: String,
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    ai_verifier: nn::Sequential,
}

impl VegaStar {
    pub fn new() -> Result<Self> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let did = format!("did:vega:{}", hex::encode(verifying_key.to_bytes()));

        let vs = nn::VarStore::new(tch::Device::Cpu);
        let ai_verifier = nn::seq()
            .add(nn::linear(&vs.root(), 8, 16, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(&vs.root(), 16, 1, Default::default()))
            .add_fn(|xs| xs.sigmoid());

        Ok(VegaStar {
            did,
            signing_key,
            verifying_key,
            ai_verifier,
        })
    }

    pub async fn load_or_create(shard_id: u64, _passphrase: &str) -> Result<Self> {
        let mut star = Self::new()?;
        star.did = format!("did:vega:{}:{}", shard_id, hex::encode(star.verifying_key.to_bytes()));
        Ok(star)
    }

    pub fn did(&self) -> &str {
        &self.did
    }

    pub async fn issue_credential(&self, subject: &str, data: Value) -> Result<Credential> {
        let issuance_date: DateTime<Utc> = Utc::now();
        let mut credential = Credential {
            context: Contexts::Many(vec![Context::URI(URI::String("https://www.w3.org/2018/credentials/v1".to_string()))]),
            id: Some(StringOrURI::String(format!("{}/cred/{}", self.did, hex::encode(&self.signing_key.to_bytes()[0..8])))),
            type_: OneOrMany::Many(vec!["VerifiableCredential".to_string()]),
            issuer: Some(Issuer::URI(URI::String(self.did.clone()))),
            issuance_date: Some(VCDateTime::from(issuance_date)),
            credential_subject: OneOrMany::One(ssi::vc::CredentialSubject {
                id: Some(URI::String(subject.to_string())),
                property_set: data.as_object().map(|obj| obj.clone().into_iter().collect()),
            }),
            credential_schema: None,
            credential_status: None,
            expiration_date: None,
            refresh_service: None,
            terms_of_use: None,
            evidence: None,
            property_set: None,
            proof: None,
        };

        let proof = ssi::ldp::Proof {
            context: Value::Null,
            creator: None,
            type_: ProofSuiteType::Ed25519Signature2020,
            created: Some(Utc::now()),
            verification_method: Some(format!("{}#key-1", self.did)),
            proof_value: Some(hex::encode(self.signing_key.sign(subject.as_bytes()).to_bytes())),
            proof_purpose: Some(VerificationRelationship::Authentication),
            challenge: None,
            domain: None,
            nonce: None,
            cryptosuite: None,
            jws: None,
            property_set: None,
        };
        credential.proof = Some(OneOrMany::One(proof));
        Ok(credential)
    }

    pub fn verify_identity(&self, behavior_data: &[u8]) -> bool {
        let tensor = Tensor::from_slice(behavior_data)
            .to_kind(tch::Kind::Float)
            .reshape(&[1, behavior_data.len() as i64]);
        let output = self.ai_verifier.forward(&tensor);
        output.double_value(&[0]) > 0.5
    }
}
