use qobject::{KPluginMetaData, QObject};

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kquickconfigmodule.h");
        type KQuickConfigModule = cxx_kde_frameworks::kcmutils::KQuickConfigModule;

        include!("cxx-kde-frameworks/kpluginmetadata.h");
        type KPluginMetaData = cxx_kde_frameworks::kcoreaddons::KPluginMetaData;

        type QObject = cxx_qt::QObject;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = KQuickConfigModule]
        type DefaultObject = super::DefaultObjectRust;

    }

    impl
        cxx_qt::Constructor<
            (*mut QObject, KPluginMetaData),
            BaseArguments = (*mut QObject, KPluginMetaData),
        > for DefaultObject
    {
    }
}

#[derive(Default)]
pub struct DefaultObjectRust {}

impl cxx_qt::Constructor<(*mut qobject::QObject, qobject::KPluginMetaData)>
    for qobject::DefaultObject
{
    type NewArguments = ();

    type BaseArguments = (*mut qobject::QObject, qobject::KPluginMetaData);

    type InitializeArguments = ();

    fn route_arguments(
        arguments: Self::BaseArguments,
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), arguments, ())
    }

    fn new(_arguments: Self::NewArguments) -> <Self as cxx_qt::CxxQtType>::Rust {
        DefaultObjectRust {}
    }
}
