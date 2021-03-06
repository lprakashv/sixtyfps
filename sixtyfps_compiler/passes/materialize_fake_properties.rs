/* LICENSE BEGIN
    This file is part of the SixtyFPS Project -- https://sixtyfps.io
    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only
    This file is also available under commercial licensing terms.
    Please contact info@sixtyfps.io for more information.
LICENSE END */
//! This pass creates properties that are used but are otherwise not real

use crate::expression_tree::NamedReference;
use crate::object_tree::*;
use crate::typeregister::Type;
use std::collections::HashMap;
use std::rc::Rc;

pub fn materialize_fake_properties(component: &Rc<Component>) {
    recurse_elem_no_borrow(&component.root_element, &(), &mut |elem, _| {
        visit_all_named_references(elem, |NamedReference { element, name }| {
            let elem = element.upgrade().unwrap();
            let elem = elem.borrow_mut();
            let (base_type, mut property_declarations) =
                std::cell::RefMut::map_split(elem, |elem| {
                    (&mut elem.base_type, &mut elem.property_declarations)
                });
            maybe_materialize(&mut property_declarations, &base_type, name);
        });
        let elem = elem.borrow_mut();
        let base_type = elem.base_type.clone();
        let (bindings, mut property_declarations) = std::cell::RefMut::map_split(elem, |elem| {
            (&mut elem.bindings, &mut elem.property_declarations)
        });
        for (prop, _) in bindings.iter() {
            maybe_materialize(&mut property_declarations, &base_type, prop);
        }
    })
}

fn maybe_materialize(
    property_declarations: &mut HashMap<String, PropertyDeclaration>,
    base_type: &Type,
    prop: &String,
) {
    if property_declarations.contains_key(prop) {
        return;
    }
    let has_declared_property = match &base_type {
        crate::typeregister::Type::Component(c) => {
            has_declared_property(&c.root_element.borrow(), prop)
        }
        crate::typeregister::Type::Builtin(b) => b.properties.contains_key(prop),
        crate::typeregister::Type::Native(n) => n.lookup_property(prop).is_some(),
        _ => false,
    };

    if !has_declared_property {
        let ty = crate::typeregister::reserved_property(prop);
        if ty != Type::Invalid {
            property_declarations.insert(
                prop.to_owned(),
                PropertyDeclaration { property_type: ty, ..PropertyDeclaration::default() },
            );
        }
    }
}

/// Returns true if the property is declared in this element or parent
/// (as opposed to being implicitly declared)
fn has_declared_property(elem: &Element, prop: &str) -> bool {
    if elem.property_declarations.contains_key(prop) {
        return true;
    }
    match &elem.base_type {
        crate::typeregister::Type::Component(c) => {
            has_declared_property(&c.root_element.borrow(), prop)
        }
        crate::typeregister::Type::Builtin(b) => b.properties.contains_key(prop),
        crate::typeregister::Type::Native(n) => n.lookup_property(prop).is_some(),
        _ => false,
    }
}
