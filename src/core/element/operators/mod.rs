use enum_dispatch::enum_dispatch;
use op_department_is::ElementOpDepartmentIs;
use op_department_is_not::ElementOpDepartmentIsNot;
use op_depends::ElementOpDepends;
use op_shot_local::ElementOpShotLocal;
use serde::{Deserialize, Serialize};

use crate::core::context::Context;

use super::{element_resolver::ResolvedElement, resolved_element_data::ResolvedElementData};

pub mod op_department_is;
pub mod op_department_is_not;
pub mod op_depends;
pub mod op_shot_local;

#[enum_dispatch]
pub trait ElementOperation {
    fn get_elements(
        &self,
        context: &Context,
        element_data: ResolvedElementData,
    ) -> Vec<ResolvedElement>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[enum_dispatch(ElementOperation)]
pub enum ElementOperator {
    DepartmentIs(ElementOpDepartmentIs),
    DepartmentIsNot(ElementOpDepartmentIsNot),
    Depends(ElementOpDepends),
    ShotLocal(ElementOpShotLocal),
}
