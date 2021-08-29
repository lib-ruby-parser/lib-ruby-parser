#[cfg(feature = "codegen-c")]
mod c;

#[cfg(feature = "codegen-cpp")]
mod cpp;

#[cfg(feature = "codegen-rust")]
mod rust;

#[cfg(feature = "codegen-y")]
mod y;

#[cfg(feature = "compile-with-external-structures")]
mod sizes;

macro_rules! codegen_if_feature {
    ($feature:literal, $codegen:block, otherwise print: $msg:literal) => {
        #[cfg(feature = $feature)]
        $codegen;
        #[cfg(not(feature = $feature))]
        println!($msg);
    };
}

pub(crate) fn codegen() {
    codegen_if_feature!(
        "codegen-y",
        { y::codegen() },
        otherwise print: "Skipping generating .y files"
    );

    codegen_if_feature!(
        "codegen-rust",
        { rust::codegen() },
        otherwise print: "Skipping generating .rs files"
    );

    codegen_if_feature!(
        "codegen-c",
        { c::codegen() },
        otherwise print: "Skipping generating .c files"
    );

    codegen_if_feature!(
        "codegen-cpp",
        { cpp::codegen() },
        otherwise print: "Skipping generating .cpp files"
    );

    codegen_if_feature!(
        "compile-with-external-structures",
        { sizes::codegen() },
        otherwise print: "Skipping generating size.rs"
    );
}
