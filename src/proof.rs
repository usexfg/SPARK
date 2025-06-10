use winterfell::math::{fields::f64::BaseElement, FieldElement, ToElements};
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin, MerkleTree},
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, Prover, TraceTable, TraceInfo,
    TransitionConstraintDegree, verify, AcceptableOptions, FieldExtension, BatchingMethod,
    DefaultTraceLde, DefaultConstraintEvaluator, DefaultConstraintCommitment, StarkDomain, PartitionOptions, TracePolyTable, CompositionPolyTrace, CompositionPoly, ConstraintCompositionCoefficients, AuxRandElements,
};

const MIN_DEPOSIT: u64 = 800; // 800 XFG

#[derive(Clone)]
pub struct PublicInputs {
    pub amount: u64,
    pub term: u32,
    pub tx_hash: [u8; 32],
    pub block_hash: [u8; 32],
    pub recipient: [u8; 32],
    pub merkle_root: [u8; 32],
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        let mut elements = vec![
            BaseElement::new(self.amount),
            BaseElement::new(self.term as u64),
        ];
        for chunk in self.tx_hash.chunks(8) {
            let mut arr = [0u8; 8];
            arr[..chunk.len()].copy_from_slice(chunk);
            elements.push(BaseElement::new(u64::from_le_bytes(arr)));
        }
        for chunk in self.block_hash.chunks(8) {
            let mut arr = [0u8; 8];
            arr[..chunk.len()].copy_from_slice(chunk);
            elements.push(BaseElement::new(u64::from_le_bytes(arr)));
        }
        for chunk in self.recipient.chunks(8) {
            let mut arr = [0u8; 8];
            arr[..chunk.len()].copy_from_slice(chunk);
            elements.push(BaseElement::new(u64::from_le_bytes(arr)));
        }
        for chunk in self.merkle_root.chunks(8) {
            let mut arr = [0u8; 8];
            arr[..chunk.len()].copy_from_slice(chunk);
            elements.push(BaseElement::new(u64::from_le_bytes(arr)));
        }
        elements
    }
}

pub struct DepositAir {
    context: AirContext<BaseElement>,
    pub_inputs: PublicInputs,
    trace_info: TraceInfo,
}

impl Air for DepositAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        let context = AirContext::new_multi_segment(
            trace_info,
            vec![TransitionConstraintDegree::new(1); trace_info.width()],
            vec![],
            0,
            8, // blowup factor
            options,
        );
        Self {
            context,
            pub_inputs,
            trace_info,
        }
    }
    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
    fn trace_info(&self) -> &TraceInfo {
        &self.trace_info
    }
    fn evaluate_transition<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        // Enforce all columns are constant
        for i in 0..frame.current().len() {
            result[i] = frame.current()[i] - frame.next()[i];
        }
    }
    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let mut assertions = vec![
            Assertion::single(0, 0, BaseElement::new(self.pub_inputs.amount)),
            Assertion::single(1, 0, BaseElement::new(self.pub_inputs.term as u64)),
        ];
        // tx_hash, block_hash, recipient, merkle_root: each is 4 field elements (8 bytes each)
        let mut col = 2;
        for arr in [
            &self.pub_inputs.tx_hash,
            &self.pub_inputs.block_hash,
            &self.pub_inputs.recipient,
            &self.pub_inputs.merkle_root,
        ] {
            for chunk in arr.chunks(8) {
                let mut bytes = [0u8; 8];
                bytes[..chunk.len()].copy_from_slice(chunk);
                assertions.push(Assertion::single(col, 0, BaseElement::new(u64::from_le_bytes(bytes))));
                col += 1;
            }
        }
        assertions
    }
}

pub struct DepositProver {
    options: ProofOptions,
}

impl DepositProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

impl Prover for DepositProver {
    type BaseField = BaseElement;
    type Air = DepositAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = Blake3_256<BaseElement>;
    type VC = MerkleTree<Self::HashFn>;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E> = DefaultTraceLde<E, Self::HashFn, Self::VC>;
    type ConstraintEvaluator<'a, E> = DefaultConstraintEvaluator<'a, Self::Air, E>;
    type ConstraintCommitment<E> = DefaultConstraintCommitment<E, Self::HashFn, Self::VC>;

    fn get_pub_inputs(&self, _trace: &Self::Trace) -> PublicInputs {
        // Not used in this example, as we pass pub_inputs directly
        PublicInputs {
            amount: 0,
            term: 0,
            tx_hash: [0u8; 32],
            block_hash: [0u8; 32],
            recipient: [0u8; 32],
            merkle_root: [0u8; 32],
        }
    }
    fn options(&self) -> &ProofOptions {
        &self.options
    }
    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self, trace_info: &TraceInfo, main_trace: &winterfell::ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>, partition_options: PartitionOptions,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain, partition_options)
    }
    fn build_constraint_commitment<E: FieldElement<BaseField = Self::BaseField>>(
        &self, composition_poly_trace: CompositionPolyTrace<E>, num_constraint_composition_columns: usize,
        domain: &StarkDomain<Self::BaseField>, partition_options: PartitionOptions,
    ) -> (Self::ConstraintCommitment<E>, CompositionPoly<E>) {
        DefaultConstraintCommitment::new(
            composition_poly_trace, num_constraint_composition_columns, domain, partition_options,
        )
    }
    fn new_evaluator<'a, E: FieldElement<BaseField = Self::BaseField>>(
        &self, air: &'a Self::Air, aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }
}

pub fn generate_proof(
    amount: u64,
    term: u32,
    tx_hash: &[u8],
    block_hash: &[u8],
    recipient: &[u8],
    merkle_root: &[u8],
) -> Result<Vec<u8>, String> {
    if amount < MIN_DEPOSIT {
        return Err("Deposit amount must be at least 80 XFG".to_string());
    }
    let options = ProofOptions::new(
        32, 8, 0, FieldExtension::None, 8, 31, BatchingMethod::Linear, BatchingMethod::Linear,
    );
    let prover = DepositProver::new(options);

    // 2 rows, 18 columns: amount, term, 4x8-byte fields (4*4=16 columns)
    let mut trace = TraceTable::new(18, 2);
    // Fill both rows with the same data
    for row in 0..2 {
        trace.set(0, row, BaseElement::new(amount));
        trace.set(1, row, BaseElement::new(term as u64));
        let mut col = 2;
        for arr in [tx_hash, block_hash, recipient, merkle_root] {
            for chunk in arr.chunks(8) {
                let mut bytes = [0u8; 8];
                bytes[..chunk.len()].copy_from_slice(chunk);
                trace.set(col, row, BaseElement::new(u64::from_le_bytes(bytes)));
                col += 1;
            }
        }
    }
    let mut tx_hash_arr = [0u8; 32];
    tx_hash_arr[..tx_hash.len().min(32)].copy_from_slice(&tx_hash[..tx_hash.len().min(32)]);
    let mut block_hash_arr = [0u8; 32];
    block_hash_arr[..block_hash.len().min(32)].copy_from_slice(&block_hash[..block_hash.len().min(32)]);
    let mut recipient_arr = [0u8; 32];
    recipient_arr[..recipient.len().min(32)].copy_from_slice(&recipient[..recipient.len().min(32)]);
    let mut merkle_root_arr = [0u8; 32];
    merkle_root_arr[..merkle_root.len().min(32)].copy_from_slice(&merkle_root[..merkle_root.len().min(32)]);
    let pub_inputs = PublicInputs {
        amount,
        term,
        tx_hash: tx_hash_arr,
        block_hash: block_hash_arr,
        recipient: recipient_arr,
        merkle_root: merkle_root_arr,
    };
    let proof = prover.prove(trace, pub_inputs.clone()).map_err(|e| format!("Prover error: {e}"))?;
    Ok(proof.to_bytes())
}

pub fn verify_proof(
    proof_bytes: &[u8],
    amount: u64,
    term: u32,
    tx_hash: &[u8],
    block_hash: &[u8],
    recipient: &[u8],
    merkle_root: &[u8],
) -> Result<bool, String> {
    if amount < MIN_DEPOSIT {
        return Err("Deposit amount must be at least 80 XFG".to_string());
    }
    let mut tx_hash_arr = [0u8; 32];
    tx_hash_arr[..tx_hash.len().min(32)].copy_from_slice(&tx_hash[..tx_hash.len().min(32)]);
    let mut block_hash_arr = [0u8; 32];
    block_hash_arr[..block_hash.len().min(32)].copy_from_slice(&block_hash[..block_hash.len().min(32)]);
    let mut recipient_arr = [0u8; 32];
    recipient_arr[..recipient.len().min(32)].copy_from_slice(&recipient[..recipient.len().min(32)]);
    let mut merkle_root_arr = [0u8; 32];
    merkle_root_arr[..merkle_root.len().min(32)].copy_from_slice(&merkle_root[..merkle_root.len().min(32)]);
    let pub_inputs = PublicInputs {
        amount,
        term,
        tx_hash: tx_hash_arr,
        block_hash: block_hash_arr,
        recipient: recipient_arr,
        merkle_root: merkle_root_arr,
    };
    let options = AcceptableOptions::MinConjecturedSecurity(95);
    let proof = winterfell::Proof::from_bytes(proof_bytes).map_err(|e| format!("Proof decode error: {e}"))?;
    let result = verify::<DepositAir, Blake3_256<BaseElement>, DefaultRandomCoin<Blake3_256<BaseElement>>, MerkleTree<Blake3_256<BaseElement>>>(proof, pub_inputs, &options);
    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
} 