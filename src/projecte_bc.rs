#![no_std]

use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum AttendanceStatus {
    NotClaimed,
    Claimed,
}

#[multiversx_sc::contract]
pub trait AttendanceTrackerSc {
    #[init]
    fn init(&self, total_classes: u64) {
        require!(total_classes > 0, "Total classes must be greater than 0");
        self.total_classes().set(total_classes);
    }

    #[endpoint]
    fn claim_attendance(&self, class_code: ManagedBuffer) {
        let caller = self.blockchain().get_caller();

        // Verifica que el codi de classe no hagi estat reclamat ja per aquest alumne
        require!(
            self.attendance_status(&caller, &class_code).get() == AttendanceStatus::NotClaimed,
            "You have already claimed attendance for this class"
        );

        // Verifica que el codi de classe sigui únic i vàlid
        require!(
            self.is_valid_class_code(&class_code),
            "Invalid class code"
        );

        // Marca l'assistència com a reclamada
        self.attendance_status(&caller, &class_code).set(AttendanceStatus::Claimed);

        // Envia una petita quantitat d'EGLD a l'alumne
        let egld_amount = BigUint::from(1_000_000_000_000_000u64); // 0.001 EGLD
        self.send().direct_egld(&caller, &egld_amount);

        // Incrementa el comptador de transaccions d'EGLD per a l'alumne
        let current_transactions = self.egld_transactions(&caller).get();
        self.egld_transactions(&caller).set(current_transactions + 1);
    }

    #[payable("EGLD")]
    #[endpoint]
    // Funció per dipositar EGLD al contracte
    fn deposit(&self) {
        let payment = self.call_value().egld_value().clone_value();
    }

    #[view(getAttendanceStatus)]
    fn get_attendance_status(&self, student: &ManagedAddress, class_code: &ManagedBuffer) -> AttendanceStatus {
        self.attendance_status(student, class_code).get()
    }

    #[view(getEgldTransactions)]
    fn get_egld_transactions(&self, student: &ManagedAddress) -> u64 {
        self.egld_transactions(student).get()
    }

    // Funció per verificar si el codi de classe és vàlid
    fn is_valid_class_code(&self, class_code: &ManagedBuffer) -> bool {
        // Un codi de classe vàlid ha de tenir exactament 6 caràcters
        class_code.len()==6
    }

    // Storage

    #[view(getTotalClasses)]
    #[storage_mapper("total_classes")]
    fn total_classes(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("attendance_status")]
    fn attendance_status(
        &self,
        student: &ManagedAddress,
        class_code: &ManagedBuffer,
    ) -> SingleValueMapper<AttendanceStatus>;

    #[storage_mapper("egld_transactions")]
    fn egld_transactions(&self, student: &ManagedAddress) -> SingleValueMapper<u64>;
}