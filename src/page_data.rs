use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct PageData {
    pub i18n: BTreeMap<&'static str, &'static str>,
    pub errors: BTreeMap<&'static str, &'static str>,
    pub form_data: BTreeMap<&'static str,String>,
}
