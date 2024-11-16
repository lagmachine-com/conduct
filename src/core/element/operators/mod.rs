use enum_dispatch::enum_dispatch;
use op_department_is::ElementOpDepartmentIs;
use op_depends::ElementOpDepends;
use serde::{Deserialize, Serialize};

use crate::core::context::Context;

use super::element::Element;

pub mod op_department_is;
pub mod op_department_is_not;
pub mod op_depends;

#[enum_dispatch]
pub trait ElementOperation {
    fn get_elements(&self, context: &Context) -> Vec<Element>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[enum_dispatch(ElementOperation)]
pub enum ElementOperator {
    DepartmentIs(ElementOpDepartmentIs),
    Depends(ElementOpDepends),
}
