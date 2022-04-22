#[cfg(feature = "codegen-rust")]
mod rust;

#[cfg(feature = "codegen-y")]
mod y;

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
}
