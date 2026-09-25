#pragma once

#include <KPluginFactory>
#include <KQuickConfigModule>

// cxx-qt 生成的 TimeSettings 声明（bridge 未使用额外命名空间，类位于全局作用域）
#include <kcm/src/lib.cxxqt.h>

K_PLUGIN_CLASS_WITH_JSON(DefaultObject, "metadata.json")
