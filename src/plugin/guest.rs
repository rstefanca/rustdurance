use std::ops::Index;
use super::pax_type::PaxType;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Guest {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) pax_type: PaxType,
    pub(crate) phone_number: Option<Phone>,
    pub(crate) email: Option<Email>,
    pub(crate) custom_fields: Vec<CustomField>,
}
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Phone {
    pub(crate) value: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Email {
    pub(crate) value: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct CustomFieldId(pub u64);

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct CustomField {
    pub(crate) id: CustomFieldId,
    pub(crate) value: String,
}

impl Guest {
    pub fn custom_field(&self, id: CustomFieldId) -> Option<&CustomField> {
        self.custom_fields.iter().find(|f| f.id == id)
    }
}

impl Index<CustomFieldId> for Guest {
    type Output = CustomField;

    /// `guest[id]` yields the custom field with that id; panics if absent (use `custom_field` to check).
    fn index(&self, id: CustomFieldId) -> &CustomField {
        self.custom_field(id)
            .unwrap_or_else(|| panic!("guest has no custom field with id {id:?}"))
    }
}
