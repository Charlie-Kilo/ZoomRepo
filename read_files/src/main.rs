use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawData {
    #[serde(default, rename = "Company Name")]
    company_name: String,
    #[serde(default, rename = "Company HQ Phone")]
    company_phone: String,
    #[serde(default, rename = "Website")]
    website: String,
    #[serde(default, rename = "Employees")]
    employee: String,
    #[serde(default, rename = "Revenue (in 000s USD)")]
    rev: String,
    #[serde(default, rename = "SIC Code 1")]
    sic_1: String,
    #[serde(default, rename = "SIC Code 2")]
    sic_2: String,
    #[serde(default, rename = "NAICS Codes")]
    naics: String,
    #[serde(default, rename = "Company Street Address")]
    company_street: String,
    #[serde(default, rename = "Company City")]
    company_city: String,
    #[serde(default, rename = "Company State")]
    company_state: String,
    #[serde(default, rename = "Company Zip Code")]
    company_code: String,
    #[serde(default, rename = "Company Country")]
    company_country: String,
    #[serde(default, rename = "First Name")]
    first_name: String,
    #[serde(default, rename = "Last Name")]
    last_name: String,
    #[serde(default, rename = "Direct Phone Number")]
    person_phone: String,
    #[serde(default, rename = "Mobile phone")]
    mobile_phone: String,
    #[serde(default, rename = "Email Address")]
    email: String,
    #[serde(default, rename = "Job Title")]
    job_title: String,
    #[serde(default, rename = "Job Function")]
    job_function: String,
    #[serde(default, rename = "Person Street")]
    person_street: String,
    #[serde(default, rename = "Person City")]
    person_city: String,
    #[serde(default, rename = "Person State")]
    person_state: String,
    #[serde(default, rename = "Person Zip Code")]
    person_code: String,
    #[serde(default, rename = "Country")]
    person_country: String,
    #[serde(default)]
    industry: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]

struct Contact {
    first_name: String,
    last_name: String,
    person_phone: String,
    mobile_phone: String,
    company_phone: String,
    email: String,
    job_title: String,
    job_function: String,
    person_street: String,
    person_city: String,
    person_state: String,
    person_code: String,
    person_country: String,
    industry: String,
    lead_source: String,
}
impl Default for Contact {
    fn default() -> Contact {
        Contact {
            first_name: String::from("missing_first_name"),
            last_name: String::from("missing_last_name"),
            person_phone: String::from("missing_phone"),
            mobile_phone: String::from("missing_mobile"),
            company_phone: String::from("missing_company_phone"),
            email: String::from("missing_email"),
            job_title: String::from("missing_job_title"),
            job_function: String::from("missing_job_function"),
            person_street: String::from("missing_street"),
            person_city: String::from("missing_city"),
            person_state: String::from("missing_state"),
            person_code: String::from("missing_code"),
            person_country: String::from("missing_country"),
            industry: String::from("No Industry"),
            lead_source: String::from("Zoom Info"),
        }
    }
}

impl Contact {
    fn create_contact(&mut self, raw_data: RawData) {
        self.first_name = raw_data.first_name;
        self.last_name = raw_data.last_name;
        self.person_phone = raw_data.person_phone;
        self.mobile_phone = raw_data.mobile_phone;
        self.company_phone = raw_data.company_phone;
        self.email = raw_data.email;
        self.job_title = raw_data.job_title;
        self.job_function = raw_data.job_function;
        self.person_street = raw_data.naics;
        self.person_street = raw_data.company_street;
        self.person_city = raw_data.company_city;
        self.person_state = raw_data.company_state;
        self.person_code = raw_data.company_code;
        self.person_country = raw_data.company_country;
        self.industry = raw_data.industry;
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Company {
    name: String,
    phone: String,
    website: String,
    employee: String,
    rev: String,
    sic_1: String,
    sic_2: String,
    naics: String,
    street: String,
    city: String,
    state: String,
    code: String,
    country: String,
    industry: String,
    lead_source: String,
}

impl Default for Company {
    fn default() -> Company {
        Company {
            name: String::from("missing_name"),
            phone: String::from("missing_phone"),
            website: String::from("missing_website"),
            employee: String::from("missing_employees"),
            rev: String::from("missing_revenue"),
            sic_1: String::from("missing_sic_1"),
            sic_2: String::from("missing_sic_codes"),
            naics: String::from("missing_naics"),
            street: String::from("missing_street"),
            city: String::from("missing_city"),
            state: String::from("missing_state"),
            code: String::from("missing_code"),
            country: String::from("missing_country"),
            industry: String::from("No Industry"),
            lead_source: String::from("Zoom Info"),
        }
    }
}

impl Company {
    fn create_company(&mut self, raw_data: RawData) {
        self.name = raw_data.company_name;
        self.phone = raw_data.company_phone;
        self.website = raw_data.website;
        self.employee = raw_data.employee;
        self.rev = raw_data.rev;
        self.sic_1 = raw_data.sic_1;
        self.sic_2 = raw_data.sic_2;
        self.naics = raw_data.naics;
        self.street = raw_data.company_street;
        self.city = raw_data.company_city;
        self.state = raw_data.company_state;
        self.code = raw_data.company_code;
        self.country = raw_data.company_country;
        self.industry = raw_data.industry;
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut companies: Vec<Company> = vec![];
    let mut contacts: Vec<Contact> = vec![];
    let mut folders: Vec<String> = vec![];
    let path = Path::new("./Upload Folder/");
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap().file_name().into_string().unwrap();
        folders.push(entry);
    }
    let complete = String::from("Complete");
    for industry in folders {
        if industry.eq(&complete) {
            continue;
        }
        let path = path.join(industry.clone());
        for result in fs::read_dir(path).unwrap() {
            let entry = result.unwrap();
            let path = entry.path();
            {
                if path.is_file() {
                    let reader = csv::ReaderBuilder::new().has_headers(true).from_path(path);
                    let _recs: Vec<RawData> = vec![];
                    for rec in reader?.deserialize() {
                        let mut new_company: Company = Company::default();
                        let mut new_contact: Contact = Contact::default();
                        let mut rec: RawData = rec?;
                        rec.industry = industry.clone().to_case(Case::Title);
                        new_contact.create_contact(rec.clone());
                        contacts.push(new_contact);
                        new_company.create_company(rec.clone());
                        companies.push(new_company);
                    }
                }
            }
        }
    }
    let wtr = &mut csv::Writer::from_path("./Upload Folder/Complete/made_in_rust_companies.csv")?;
    for company in companies {
        wtr.serialize(company)?;
    }
    wtr.flush()?;
    let wtr = &mut csv::Writer::from_path("./Upload Folder/Complete/made_in_rust_contacts.csv")?;
    for contact in contacts {
        wtr.serialize(contact)?;
    }
    wtr.flush()?;
    Ok(())
}
