use std::fmt;
use model::fi::lar::lar_id::LarIdentifier;
use model::fi::lar::loan::Loan;
use model::fi::lar::lar_action::LarAction;
use model::fi::lar::geography::Geography;
use model::fi::lar::applicant::Applicant;
use model::fi::lar::denial::Denial;
use model::fi::lar::loan_disclosure::LoanDisclosure;
use model::fi::lar::non_amortizing_features::NonAmortizingFeatures;
use model::fi::lar::property::Property;
use model::fi::lar::aus::AutomatedUnderwritingSystem;
use model::fi::lar::aus_result::AutomatedUnderwritingSystemResult;

pub struct LoanApplicationRegister {
    pub lar_identifier: LarIdentifier,
    pub loan: Loan,
    pub action: LarAction,
    pub geography: Geography,
    pub applicant: Applicant,
    pub co_applicant: Applicant,
    pub income: String,
    pub purchaser_type: i8,
    pub hoepa_status: i8,
    pub lien_status: i8,
    pub denial: Denial,
    pub loan_disclosure: LoanDisclosure,
    pub non_amortizing_features: NonAmortizingFeatures,
    pub property: Property,
    pub application_submission: i8,
    pub payable_to_institution: i8,
    pub aus: AutomatedUnderwritingSystem,
    pub aus_result: AutomatedUnderwritingSystemResult,
    pub reverse_mortgage: i8,
    pub line_of_credit: i8,
    pub business_or_commercial_purpose: i8,
}

impl LoanApplicationRegister {
    pub fn lar_sample() -> LoanApplicationRegister {
        LoanApplicationRegister {
            lar_identifier: LarIdentifier::sample_lar_identifier(),
            loan: Loan::sample_loan(),
            action: LarAction::sample_lar_action(),
            geography: Geography::sample_geography(),
            applicant: Applicant::sample_applicant(),
            co_applicant: Applicant::sample_applicant(),
            income: "36".to_string(),
            purchaser_type: 1,
            hoepa_status: 1,
            lien_status: 1,
            denial: Denial::sample_denial(),
            loan_disclosure: LoanDisclosure::sample_loan_disclosure(),
            non_amortizing_features: NonAmortizingFeatures::sample_non_amortizing_features(),
            property: Property::sample_property(),
            application_submission: 1,
            payable_to_institution: 1,
            aus: AutomatedUnderwritingSystem::sample_aus(),
            aus_result: AutomatedUnderwritingSystemResult::sample_aus_result(),
            reverse_mortgage: 1,
            line_of_credit: 1,
            business_or_commercial_purpose: 1,
        }
    }
}

impl fmt::Display for LoanApplicationRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.lar_identifier.id,
            self.lar_identifier.lei,
            self.loan.uli,
            self.loan.application_date,
            self.loan.loan_type,
            self.loan.loan_purpose,
            self.action.preapproval,
            self.loan.construction_method,
            self.loan.occupancy,
            self.loan.amount,
            self.action.action_taken_type,
            self.action.action_taken_date,
            self.geography.street,
            self.geography.city,
            self.geography.state,
            self.geography.zip_code,
            self.geography.county,
            self.geography.tract,
            self.applicant.ethnicity.ethnicity1,
            self.applicant.ethnicity.ethnicity2,
            self.applicant.ethnicity.ethnicity3,
            self.applicant.ethnicity.ethnicity4,
            self.applicant.ethnicity.ethnicity5,
            self.applicant.ethnicity.other_hispanic_latino,
            self.co_applicant.ethnicity.ethnicity1,
            self.co_applicant.ethnicity.ethnicity2,
            self.co_applicant.ethnicity.ethnicity3,
            self.co_applicant.ethnicity.ethnicity4,
            self.co_applicant.ethnicity.ethnicity5,
            self.co_applicant.ethnicity.other_hispanic_latino,
            self.applicant.ethnicity.ethnicity_observed,
            self.co_applicant.ethnicity.ethnicity_observed,
            self.applicant.race.race1,
            self.applicant.race.race2,
            self.applicant.race.race3,
            self.applicant.race.race4,
            self.applicant.race.race5,
            self.applicant.race.other_native,
            self.applicant.race.other_asian,
            self.applicant.race.other_pacific_islander,
            self.co_applicant.race.race1,
            self.co_applicant.race.race2,
            self.co_applicant.race.race3,
            self.co_applicant.race.race4,
            self.co_applicant.race.race5,
            self.co_applicant.race.other_native,
            self.co_applicant.race.other_asian,
            self.co_applicant.race.other_pacific_islander,
            self.applicant.race.race_observed,
            self.co_applicant.race.race_observed,
            self.applicant.sex.sex,
            self.co_applicant.sex.sex,
            self.applicant.sex.sex_observed,
            self.co_applicant.sex.sex_observed,
            self.applicant.age,
            self.co_applicant.age,
            self.income,
            self.purchaser_type,
            self.loan.rate_spread,
            self.hoepa_status,
            self.lien_status,
            self.applicant.credit_score,
            self.co_applicant.credit_score,
            self.applicant.credit_score_type,
            self.applicant.other_credit_score_model,
            self.co_applicant.credit_score_type,
            self.co_applicant.other_credit_score_model,
            self.denial.denial_reason_1,
            self.denial.denial_reason_2,
            self.denial.denial_reason_3,
            self.denial.denial_reason_4,
            self.denial.other_denial_reason,
            self.loan_disclosure.total_loan_costs,
            self.loan_disclosure.total_points_and_fees,
            self.loan_disclosure.origination_charges,
            self.loan_disclosure.discount_points,
            self.loan_disclosure.lender_credits,
            self.loan.interest_rate,
            self.loan.prepayment_penalty_term,
            self.loan.debt_to_income_ratio,
            self.loan.loan_to_value_ratio,
            self.loan.loan_term,
            self.loan.introductory_rate_period,
            self.non_amortizing_features.balloon_payment,
            self.non_amortizing_features.interest_only_payment,
            self.non_amortizing_features.negative_amortization,
            self.non_amortizing_features.other_non_amortizing_features,
            self.property.property_value,
            self.property.manufactured_home_secured_property,
            self.property.manufactured_home_land_property_interest,
            self.property.total_units,
            self.property.multifamily_affordable_units,
            self.application_submission,
            self.payable_to_institution,
            self.lar_identifier.nmls_identifier,
            self.aus.aus1,
            self.aus.aus2,
            self.aus.aus3,
            self.aus.aus4,
            self.aus.aus5,
            self.aus.other_aus,
            self.aus_result.aus_result1,
            self.aus_result.aus_result2,
            self.aus_result.aus_result3,
            self.aus_result.aus_result4,
            self.aus_result.aus_result5,
            self.aus_result.other_aus_result,
            self.reverse_mortgage,
            self.line_of_credit,
            self.business_or_commercial_purpose
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lar_sample() {
        let lar = LoanApplicationRegister::lar_sample();

        assert_eq!(lar.lar_identifier.id, 2);

        assert_eq!(
            lar.to_string(),
                "2|10Bx939c5543TqA1144M|10Bx939c5543TqA1144M999143X38|20180721|1|1|1|1|1|110500|1|20180721|123 Main St|Beverly Hills|CA|90210|06037|06037264000|1|1|1|1|1||1|1|1|1|1||3|3|5|7|7|7|7||||5|7|7|7|7||||3|3|1|1|3|3|30|30|36|1|0.428|1|1|750|750|1|9|1|9|-1|-1|-1|-1||2399.04|NA|NA|NA|NA|4.125|NA|42.95|80.05|360|NA|1|2|1|1|350500|1|1|5|NA|1|1|12345|1|1|1|1|1||1|1|1|1|1||1|1|1"
                )
    }

}
