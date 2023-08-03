#include "cxx-qt-gen/inheritance.cxxqt.h"

MyObject::~MyObject() {}

QVariant
MyObject::data(QModelIndex const& _index, ::std::int32_t _role) const
{
  const auto guard = unsafeRustLock();
  return dataWrapper(_index, _role);
}

bool
MyObject::hasChildren(QModelIndex const& _parent) const
{
  const auto guard = unsafeRustLock();
  return hasChildrenWrapper(_parent);
}

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(::cxx_qt_my_object::createRs())
{
}
