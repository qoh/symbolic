//! Swift Demangling Tests
//! All functions were compiled with Swift 4.0 in a file called mangling.swift
//! see https://github.com/apple/swift/blob/master/test/SILGen/mangling.swift
#![allow(clippy::cognitive_complexity)]

#[macro_use]
mod utils;

use symbolic_common::Language;

#[test]
fn test_demangle_swift_short() {
    assert_demangle!(Language::Swift, utils::WITH_ARGS, {
        // Swift < 4 (old mangling)
        "_T08mangling0022egbpdajGbuEbxfgehfvwxnyyF" => "ليهمابتكلموشعربي؟()",
        "_T08mangling0024ihqwcrbEcvIaIdqgAFGpqjyeyyF" => "他们为什么不说中文()",
        "_T08mangling0027ihqwctvzcJBfGFJdrssDxIboAybyyF" => "他們爲什麽不說中文()",
        "_T08mangling0030Proprostnemluvesky_uybCEdmaEBayyF" => "Pročprostěnemluvíčesky()",
        "_T08mangling9r13757744ySaySiG1x_tF" => "r13757744(x:)",
        "_T08mangling9r13757744ySi1xd_tF" => "r13757744(x:)",
        "_T08mangling2psopyxlF" => "+- prefix<A>(_:)",
        "_T08mangling2psoPyxlF" => "+- postfix<A>(_:)",
        "_T08mangling2psoiyx_xtlF" => "+- infix<A>(_:_:)",
        "_T08mangling2psopyx1a_x1bt_tlF" => "+- prefix<A>(_:)",
        "_T08mangling2psoPyx1a_x1bt_tlF" => "+- postfix<A>(_:)",
        "_T08mangling007p_qcaDcoiS2i_SitF" => "«+» infix(_:_:)",
        "_T08mangling12any_protocolyypF" => "any_protocol(_:)",
        "_T08mangling12one_protocolyAA3Foo_pF" => "one_protocol(_:)",
        "_T08mangling18one_protocol_twiceyAA3Foo_p_AaC_ptF" => "one_protocol_twice(_:_:)",
        "_T08mangling12two_protocolyAA3Bar_AA3FoopF" => "two_protocol(_:)",
        "_T08mangling3ZimC4zangyx_qd__tlF" => "Zim.zang<A>(_:_:)",
        "_T08mangling3ZimC4zungyqd___xtlF" => "Zim.zung<A>(_:_:)",
        "_T08mangling27single_protocol_compositionyAA3Foo_p1x_tF" => "single_protocol_composition(x:)",
        "_T08mangling28uses_objc_class_and_protocolySo8NSObjectC1o_So8NSAnsing_p1ptF" => "uses_objc_class_and_protocol(o:p:)",
        "_T08mangling17uses_clang_structySC6NSRectV1r_tF" => "uses_clang_struct(r:)",
        "_T08mangling14uses_optionalss7UnicodeO6ScalarVSgSiSg1x_tF" => "uses_optionals(x:)",
        "_T08mangling12GenericUnionO3FooACyxGSicAEmlF" => "GenericUnion.Foo<A>(_:)",
        "_T08mangling10HasVarInitV5stateSbvpZfiSbyKXKfu_" => "implicit closure #1 in variable initialization expression of static HasVarInit.state",
        "_T08mangling19autoClosureOverloadySiyXK1f_tF" => "autoClosureOverload(f:)",
        "_T08mangling19autoClosureOverloadySiyc1f_tF" => "autoClosureOverload(f:)",
        "_T08mangling24autoClosureOverloadCallsyyF" => "autoClosureOverloadCalls()",
        "_T08mangling4fooAyxAA12HasAssocTypeRzlF" => "fooA<A>(_:)",
        "_T08mangling4fooByxAA12HasAssocTypeRzAA0D4Reqt0D0RpzlF" => "fooB<A>(_:)",
        "_T08mangling2qqoiySi_SitF" => "?? infix(_:_:)",
        "_T08mangling24InstanceAndClassPropertyV8propertySivg" => "InstanceAndClassProperty.property.getter",
        "_T08mangling24InstanceAndClassPropertyV8propertySivs" => "InstanceAndClassProperty.property.setter",
        "_T08mangling24InstanceAndClassPropertyV8propertySivgZ" => "static InstanceAndClassProperty.property.getter",
        "_T08mangling24InstanceAndClassPropertyV8propertySivsZ" => "static InstanceAndClassProperty.property.setter",
        "_T08mangling6curry1yyF" => "curry1()",
        "_T08mangling3barSiyKF" => "bar()",
        "_T08mangling12curry1ThrowsyyKF" => "curry1Throws()",
        "_T08mangling12curry2ThrowsyycyKF" => "curry2Throws()",
        "_T08mangling6curry3yyKcyF" => "curry3()",
        "_T08mangling12curry3ThrowsyyKcyKF" => "curry3Throws()",
        "_T08mangling14varargsVsArrayySi3arrd_SS1ntF" => "varargsVsArray(arr:n:)",
        "_T08mangling14varargsVsArrayySaySiG3arr_SS1ntF" => "varargsVsArray(arr:n:)",
        "_T08mangling14varargsVsArrayySaySiG3arrd_SS1ntF" => "varargsVsArray(arr:n:)",

        // Swift 4.2
        "$S8mangling0022egbpdajGbuEbxfgehfvwxnyyF" => "ليهمابتكلموشعربي؟()",
        "$S8mangling0024ihqwcrbEcvIaIdqgAFGpqjyeyyF" => "他们为什么不说中文()",
        "$S8mangling0027ihqwctvzcJBfGFJdrssDxIboAybyyF" => "他們爲什麽不說中文()",
        "$S8mangling0030Proprostnemluvesky_uybCEdmaEBayyF" => "Pročprostěnemluvíčesky()",
        "$S8mangling9r137577441xySaySiG_tF" => "r13757744(x:)",
        "$S8mangling9r137577441xySid_tF" => "r13757744(x:)",
        "$S8mangling2psopyyxlF" => "+- prefix<A>(_:)",
        "$S8mangling2psoPyyxlF" => "+- postfix<A>(_:)",
        "$S8mangling2psoiyyx_xtlF" => "+- infix<A>(_:_:)",
        "$S8mangling2psopyyx1a_x1bt_tlF" => "+- prefix<A>(_:)",
        "$S8mangling2psoPyyx1a_x1bt_tlF" => "+- postfix<A>(_:)",
        "$S8mangling007p_qcaDcoiyS2i_SitF" => "«+» infix(_:_:)",
        "$S8mangling12any_protocolyyypF" => "any_protocol(_:)",
        "$S8mangling12one_protocolyyAA3Foo_pF" => "one_protocol(_:)",
        "$S8mangling18one_protocol_twiceyyAA3Foo_p_AaC_ptF" => "one_protocol_twice(_:_:)",
        "$S8mangling12two_protocolyyAA3Bar_AA3FoopF" => "two_protocol(_:)",
        "$S8mangling3ZimC4zangyyx_qd__tlF" => "Zim.zang<A>(_:_:)",
        "$S8mangling3ZimC4zungyyqd___xtlF" => "Zim.zung<A>(_:_:)",
        "$S8mangling28uses_objc_class_and_protocol1o1p2p2ySo8NSObjectC_So8NSAnsing_pSo14NSBetterAnsing_ptF" => "uses_objc_class_and_protocol(o:p:p2:)",
        "$S8mangling17uses_clang_struct1rySo6NSRectV_tF" => "uses_clang_struct(r:)",
        "$S8mangling14uses_optionals1xs7UnicodeO6ScalarVSgSiSg_tF" => "uses_optionals(x:)",
        "$S8mangling12GenericUnionO3FooyACyxGSicAEmlF" => "GenericUnion.Foo<A>(_:)",
        "$S8mangling10HasVarInitV5stateSbvpZfiSbyKXKfu_" => "implicit closure #1 in variable initialization expression of static HasVarInit.state",
        "$S8mangling19autoClosureOverload1fySiyXK_tF" => "autoClosureOverload(f:)",
        "$S8mangling19autoClosureOverload1fySiyXE_tF" => "autoClosureOverload(f:)",
        "$S8mangling24autoClosureOverloadCallsyyF" => "autoClosureOverloadCalls()",
        "$S8mangling4fooAyyxAA12HasAssocTypeRzlF" => "fooA<A>(_:)",
        "$S8mangling4fooByyxAA12HasAssocTypeRzAA0D4Reqt0D0RpzlF" => "fooB<A>(_:)",
        "$S8mangling2qqoiyySi_SitF" => "?? infix(_:_:)",
        "$S8mangling24InstanceAndClassPropertyV8propertySivg" => "InstanceAndClassProperty.property.getter",
        "$S8mangling24InstanceAndClassPropertyV8propertySivs" => "InstanceAndClassProperty.property.setter",
        "$S8mangling24InstanceAndClassPropertyV8propertySivgZ" => "static InstanceAndClassProperty.property.getter",
        "$S8mangling24InstanceAndClassPropertyV8propertySivsZ" => "static InstanceAndClassProperty.property.setter",
        "$S8mangling6curry1yyF" => "curry1()",
        "$S8mangling3barSiyKF" => "bar()",
        "$S8mangling12curry1ThrowsyyKF" => "curry1Throws()",
        "$S8mangling12curry2ThrowsyycyKF" => "curry2Throws()",
        "$S8mangling6curry3yyKcyF" => "curry3()",
        "$S8mangling12curry3ThrowsyyKcyKF" => "curry3Throws()",
        "$S8mangling14varargsVsArray3arr1nySid_SStF" => "varargsVsArray(arr:n:)",
        "$S8mangling14varargsVsArray3arr1nySaySiG_SStF" => "varargsVsArray(arr:n:)",
        "$S8mangling14varargsVsArray3arr1nySaySiGd_SStF" => "varargsVsArray(arr:n:)",

        // Swift 5
        "$s8mangling0022egbpdajGbuEbxfgehfvwxnyyF" => "ليهمابتكلموشعربي؟()",
        "$s8mangling0024ihqwcrbEcvIaIdqgAFGpqjyeyyF" => "他们为什么不说中文()",
        "$s8mangling0027ihqwctvzcJBfGFJdrssDxIboAybyyF" => "他們爲什麽不說中文()",
        "$s8mangling0030Proprostnemluvesky_uybCEdmaEBayyF" => "Pročprostěnemluvíčesky()",
        "$s8mangling9r137577441xySaySiG_tF" => "r13757744(x:)",
        "$s8mangling9r137577441xySid_tF" => "r13757744(x:)",
        "$s8mangling2psopyyxlF" => "+- prefix<A>(_:)",
        "$s8mangling2psoPyyxlF" => "+- postfix<A>(_:)",
        "$s8mangling2psoiyyx_xtlF" => "+- infix<A>(_:_:)",
        "$s8mangling2psopyyx1a_x1bt_tlF" => "+- prefix<A>(_:)",
        "$s8mangling2psoPyyx1a_x1bt_tlF" => "+- postfix<A>(_:)",
        "$s8mangling007p_qcaDcoiyS2i_SitF" => "«+» infix(_:_:)",
        "$s8mangling12any_protocolyyypF" => "any_protocol(_:)",
        "$s8mangling12one_protocolyyAA3Foo_pF" => "one_protocol(_:)",
        "$s8mangling18one_protocol_twiceyyAA3Foo_p_AaC_ptF" => "one_protocol_twice(_:_:)",
        "$s8mangling12two_protocolyyAA3Bar_AA3FoopF" => "two_protocol(_:)",
        "$s8mangling3ZimC4zangyyx_qd__tlF" => "Zim.zang<A>(_:_:)",
        "$s8mangling3ZimC4zungyyqd___xtlF" => "Zim.zung<A>(_:_:)",
        "$s8mangling28uses_objc_class_and_protocol1o1p2p2ySo8NSObjectC_So8NSAnsing_pSo14NSBetterAnsing_ptF" => "uses_objc_class_and_protocol(o:p:p2:)",
        "$s8mangling17uses_clang_struct1rySo6NSRectV_tF" => "uses_clang_struct(r:)",
        "$s8mangling14uses_optionals1xs7UnicodeO6ScalarVSgSiSg_tF" => "uses_optionals(x:)",
        "$s8mangling12GenericUnionO3FooyACyxGSicAEmlF" => "GenericUnion.Foo<A>(_:)",
        "$s8mangling10HasVarInitV5stateSbvpZfiSbyKXKfu_" => "implicit closure #1 in variable initialization expression of static HasVarInit.state",
        "$s8mangling19autoClosureOverload1fySiyXK_tF" => "autoClosureOverload(f:)",
        "$s8mangling19autoClosureOverload1fySiyXE_tF" => "autoClosureOverload(f:)",
        "$s8mangling24autoClosureOverloadCallsyyF" => "autoClosureOverloadCalls()",
        "$s8mangling4fooAyyxAA12HasAssocTypeRzlF" => "fooA<A>(_:)",
        "$s8mangling4fooByyxAA12HasAssocTypeRzAA0D4Reqt0D0RpzlF" => "fooB<A>(_:)",
        "$s8mangling2qqoiyySi_SitF" => "?? infix(_:_:)",
        "$s8mangling24InstanceAndClassPropertyV8propertySivg" => "InstanceAndClassProperty.property.getter",
        "$s8mangling24InstanceAndClassPropertyV8propertySivs" => "InstanceAndClassProperty.property.setter",
        "$s8mangling24InstanceAndClassPropertyV8propertySivgZ" => "static InstanceAndClassProperty.property.getter",
        "$s8mangling24InstanceAndClassPropertyV8propertySivsZ" => "static InstanceAndClassProperty.property.setter",
        "$s8mangling6curry1yyF" => "curry1()",
        "$s8mangling3barSiyKF" => "bar()",
        "$s8mangling12curry1ThrowsyyKF" => "curry1Throws()",
        "$s8mangling12curry2ThrowsyycyKF" => "curry2Throws()",
        "$s8mangling6curry3yyKcyF" => "curry3()",
        "$s8mangling12curry3ThrowsyyKcyKF" => "curry3Throws()",
        "$s8mangling14varargsVsArray3arr1nySid_SStF" => "varargsVsArray(arr:n:)",
        "$s8mangling14varargsVsArray3arr1nySaySiG_SStF" => "varargsVsArray(arr:n:)",
        "$s8mangling14varargsVsArray3arr1nySaySiGd_SStF" => "varargsVsArray(arr:n:)",
    });
}

#[test]
fn test_demangle_swift_no_args() {
    assert_demangle!(Language::Swift, utils::WITHOUT_ARGS, {
        // Swift < 4 (old mangling)
        "_T08mangling0022egbpdajGbuEbxfgehfvwxnyyF" => "ليهمابتكلموشعربي؟",
        "_T08mangling0024ihqwcrbEcvIaIdqgAFGpqjyeyyF" => "他们为什么不说中文",
        "_T08mangling0027ihqwctvzcJBfGFJdrssDxIboAybyyF" => "他們爲什麽不說中文",
        "_T08mangling0030Proprostnemluvesky_uybCEdmaEBayyF" => "Pročprostěnemluvíčesky",
        "_T08mangling9r13757744ySaySiG1x_tF" => "r13757744",
        "_T08mangling9r13757744ySi1xd_tF" => "r13757744",
        "_T08mangling2psopyxlF" => "+- prefix<A>",
        "_T08mangling2psoPyxlF" => "+- postfix<A>",
        "_T08mangling2psoiyx_xtlF" => "+- infix<A>",
        "_T08mangling2psopyx1a_x1bt_tlF" => "+- prefix<A>",
        "_T08mangling2psoPyx1a_x1bt_tlF" => "+- postfix<A>",
        "_T08mangling007p_qcaDcoiS2i_SitF" => "«+» infix",
        "_T08mangling12any_protocolyypF" => "any_protocol",
        "_T08mangling12one_protocolyAA3Foo_pF" => "one_protocol",
        "_T08mangling18one_protocol_twiceyAA3Foo_p_AaC_ptF" => "one_protocol_twice",
        "_T08mangling12two_protocolyAA3Bar_AA3FoopF" => "two_protocol",
        "_T08mangling3ZimC4zangyx_qd__tlF" => "Zim.zang<A>",
        "_T08mangling3ZimC4zungyqd___xtlF" => "Zim.zung<A>",
        "_T08mangling27single_protocol_compositionyAA3Foo_p1x_tF" => "single_protocol_composition",
        "_T08mangling28uses_objc_class_and_protocolySo8NSObjectC1o_So8NSAnsing_p1ptF" => "uses_objc_class_and_protocol",
        "_T08mangling17uses_clang_structySC6NSRectV1r_tF" => "uses_clang_struct",
        "_T08mangling14uses_optionalss7UnicodeO6ScalarVSgSiSg1x_tF" => "uses_optionals",
        "_T08mangling12GenericUnionO3FooACyxGSicAEmlF" => "GenericUnion.Foo<A>",
        "_T08mangling10HasVarInitV5stateSbvpZfiSbyKXKfu_" => "implicit closure #1 in variable initialization expression of static HasVarInit.state",
        "_T08mangling19autoClosureOverloadySiyXK1f_tF" => "autoClosureOverload",
        "_T08mangling19autoClosureOverloadySiyc1f_tF" => "autoClosureOverload",
        "_T08mangling24autoClosureOverloadCallsyyF" => "autoClosureOverloadCalls",
        "_T08mangling4fooAyxAA12HasAssocTypeRzlF" => "fooA<A>",
        "_T08mangling4fooByxAA12HasAssocTypeRzAA0D4Reqt0D0RpzlF" => "fooB<A>",
        "_T08mangling2qqoiySi_SitF" => "?? infix",
        "_T08mangling24InstanceAndClassPropertyV8propertySivg" => "InstanceAndClassProperty.property.getter",
        "_T08mangling24InstanceAndClassPropertyV8propertySivs" => "InstanceAndClassProperty.property.setter",
        "_T08mangling24InstanceAndClassPropertyV8propertySivgZ" => "static InstanceAndClassProperty.property.getter",
        "_T08mangling24InstanceAndClassPropertyV8propertySivsZ" => "static InstanceAndClassProperty.property.setter",
        "_T08mangling6curry1yyF" => "curry1",
        "_T08mangling3barSiyKF" => "bar",
        "_T08mangling12curry1ThrowsyyKF" => "curry1Throws",
        "_T08mangling12curry2ThrowsyycyKF" => "curry2Throws",
        "_T08mangling6curry3yyKcyF" => "curry3",
        "_T08mangling12curry3ThrowsyyKcyKF" => "curry3Throws",
        "_T08mangling14varargsVsArrayySi3arrd_SS1ntF" => "varargsVsArray",
        "_T08mangling14varargsVsArrayySaySiG3arr_SS1ntF" => "varargsVsArray",
        "_T08mangling14varargsVsArrayySaySiG3arrd_SS1ntF" => "varargsVsArray",

        // Swift 4.2
        "$S8mangling0022egbpdajGbuEbxfgehfvwxnyyF" => "ليهمابتكلموشعربي؟",
        "$S8mangling0024ihqwcrbEcvIaIdqgAFGpqjyeyyF" => "他们为什么不说中文",
        "$S8mangling0027ihqwctvzcJBfGFJdrssDxIboAybyyF" => "他們爲什麽不說中文",
        "$S8mangling0030Proprostnemluvesky_uybCEdmaEBayyF" => "Pročprostěnemluvíčesky",
        "$S8mangling9r137577441xySaySiG_tF" => "r13757744",
        "$S8mangling9r137577441xySid_tF" => "r13757744",
        "$S8mangling2psopyyxlF" => "+- prefix<A>",
        "$S8mangling2psoPyyxlF" => "+- postfix<A>",
        "$S8mangling2psoiyyx_xtlF" => "+- infix<A>",
        "$S8mangling2psopyyx1a_x1bt_tlF" => "+- prefix<A>",
        "$S8mangling2psoPyyx1a_x1bt_tlF" => "+- postfix<A>",
        "$S8mangling007p_qcaDcoiyS2i_SitF" => "«+» infix",
        "$S8mangling12any_protocolyyypF" => "any_protocol",
        "$S8mangling12one_protocolyyAA3Foo_pF" => "one_protocol",
        "$S8mangling18one_protocol_twiceyyAA3Foo_p_AaC_ptF" => "one_protocol_twice",
        "$S8mangling12two_protocolyyAA3Bar_AA3FoopF" => "two_protocol",
        "$S8mangling3ZimC4zangyyx_qd__tlF" => "Zim.zang<A>",
        "$S8mangling3ZimC4zungyyqd___xtlF" => "Zim.zung<A>",
        "$S8mangling28uses_objc_class_and_protocol1o1p2p2ySo8NSObjectC_So8NSAnsing_pSo14NSBetterAnsing_ptF" => "uses_objc_class_and_protocol",
        "$S8mangling17uses_clang_struct1rySo6NSRectV_tF" => "uses_clang_struct",
        "$S8mangling14uses_optionals1xs7UnicodeO6ScalarVSgSiSg_tF" => "uses_optionals",
        "$S8mangling12GenericUnionO3FooyACyxGSicAEmlF" => "GenericUnion.Foo<A>",
        "$S8mangling10HasVarInitV5stateSbvpZfiSbyKXKfu_" => "implicit closure #1 in variable initialization expression of static HasVarInit.state",
        "$S8mangling19autoClosureOverload1fySiyXK_tF" => "autoClosureOverload",
        "$S8mangling19autoClosureOverload1fySiyXE_tF" => "autoClosureOverload",
        "$S8mangling24autoClosureOverloadCallsyyF" => "autoClosureOverloadCalls",
        "$S8mangling4fooAyyxAA12HasAssocTypeRzlF" => "fooA<A>",
        "$S8mangling4fooByyxAA12HasAssocTypeRzAA0D4Reqt0D0RpzlF" => "fooB<A>",
        "$S8mangling2qqoiyySi_SitF" => "?? infix",
        "$S8mangling24InstanceAndClassPropertyV8propertySivg" => "InstanceAndClassProperty.property.getter",
        "$S8mangling24InstanceAndClassPropertyV8propertySivs" => "InstanceAndClassProperty.property.setter",
        "$S8mangling24InstanceAndClassPropertyV8propertySivgZ" => "static InstanceAndClassProperty.property.getter",
        "$S8mangling24InstanceAndClassPropertyV8propertySivsZ" => "static InstanceAndClassProperty.property.setter",
        "$S8mangling6curry1yyF" => "curry1",
        "$S8mangling3barSiyKF" => "bar",
        "$S8mangling12curry1ThrowsyyKF" => "curry1Throws",
        "$S8mangling12curry2ThrowsyycyKF" => "curry2Throws",
        "$S8mangling6curry3yyKcyF" => "curry3",
        "$S8mangling12curry3ThrowsyyKcyKF" => "curry3Throws",
        "$S8mangling14varargsVsArray3arr1nySid_SStF" => "varargsVsArray",
        "$S8mangling14varargsVsArray3arr1nySaySiG_SStF" => "varargsVsArray",
        "$S8mangling14varargsVsArray3arr1nySaySiGd_SStF" => "varargsVsArray",

        // Swift 5
        "$s8mangling0022egbpdajGbuEbxfgehfvwxnyyF" => "ليهمابتكلموشعربي؟",
        "$s8mangling0024ihqwcrbEcvIaIdqgAFGpqjyeyyF" => "他们为什么不说中文",
        "$s8mangling0027ihqwctvzcJBfGFJdrssDxIboAybyyF" => "他們爲什麽不說中文",
        "$s8mangling0030Proprostnemluvesky_uybCEdmaEBayyF" => "Pročprostěnemluvíčesky",
        "$s8mangling9r137577441xySaySiG_tF" => "r13757744",
        "$s8mangling9r137577441xySid_tF" => "r13757744",
        "$s8mangling2psopyyxlF" => "+- prefix<A>",
        "$s8mangling2psoPyyxlF" => "+- postfix<A>",
        "$s8mangling2psoiyyx_xtlF" => "+- infix<A>",
        "$s8mangling2psopyyx1a_x1bt_tlF" => "+- prefix<A>",
        "$s8mangling2psoPyyx1a_x1bt_tlF" => "+- postfix<A>",
        "$s8mangling007p_qcaDcoiyS2i_SitF" => "«+» infix",
        "$s8mangling12any_protocolyyypF" => "any_protocol",
        "$s8mangling12one_protocolyyAA3Foo_pF" => "one_protocol",
        "$s8mangling18one_protocol_twiceyyAA3Foo_p_AaC_ptF" => "one_protocol_twice",
        "$s8mangling12two_protocolyyAA3Bar_AA3FoopF" => "two_protocol",
        "$s8mangling3ZimC4zangyyx_qd__tlF" => "Zim.zang<A>",
        "$s8mangling3ZimC4zungyyqd___xtlF" => "Zim.zung<A>",
        "$s8mangling28uses_objc_class_and_protocol1o1p2p2ySo8NSObjectC_So8NSAnsing_pSo14NSBetterAnsing_ptF" => "uses_objc_class_and_protocol",
        "$s8mangling17uses_clang_struct1rySo6NSRectV_tF" => "uses_clang_struct",
        "$s8mangling14uses_optionals1xs7UnicodeO6ScalarVSgSiSg_tF" => "uses_optionals",
        "$s8mangling12GenericUnionO3FooyACyxGSicAEmlF" => "GenericUnion.Foo<A>",
        "$s8mangling10HasVarInitV5stateSbvpZfiSbyKXKfu_" => "implicit closure #1 in variable initialization expression of static HasVarInit.state",
        "$s8mangling19autoClosureOverload1fySiyXK_tF" => "autoClosureOverload",
        "$s8mangling19autoClosureOverload1fySiyXE_tF" => "autoClosureOverload",
        "$s8mangling24autoClosureOverloadCallsyyF" => "autoClosureOverloadCalls",
        "$s8mangling4fooAyyxAA12HasAssocTypeRzlF" => "fooA<A>",
        "$s8mangling4fooByyxAA12HasAssocTypeRzAA0D4Reqt0D0RpzlF" => "fooB<A>",
        "$s8mangling2qqoiyySi_SitF" => "?? infix",
        "$s8mangling24InstanceAndClassPropertyV8propertySivg" => "InstanceAndClassProperty.property.getter",
        "$s8mangling24InstanceAndClassPropertyV8propertySivs" => "InstanceAndClassProperty.property.setter",
        "$s8mangling24InstanceAndClassPropertyV8propertySivgZ" => "static InstanceAndClassProperty.property.getter",
        "$s8mangling24InstanceAndClassPropertyV8propertySivsZ" => "static InstanceAndClassProperty.property.setter",
        "$s8mangling6curry1yyF" => "curry1",
        "$s8mangling3barSiyKF" => "bar",
        "$s8mangling12curry1ThrowsyyKF" => "curry1Throws",
        "$s8mangling12curry2ThrowsyycyKF" => "curry2Throws",
        "$s8mangling6curry3yyKcyF" => "curry3",
        "$s8mangling12curry3ThrowsyyKcyKF" => "curry3Throws",
        "$s8mangling14varargsVsArray3arr1nySid_SStF" => "varargsVsArray",
        "$s8mangling14varargsVsArray3arr1nySaySiG_SStF" => "varargsVsArray",
        "$s8mangling14varargsVsArray3arr1nySaySiGd_SStF" => "varargsVsArray",
    });
}
