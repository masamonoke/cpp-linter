auto uptr = std::make_unique<int>(); // 1
auto pUptr = std::make_unique<int>(); // 0
auto ptr = std::make_unique<int>(); // 1
auto uptr = std::make_unique<SomeType>(); // 1
auto pUptr = std::make_unique<SomeType>(); // 0
auto ptr = std::make_unique<SomeType>(); // 1
auto ptr = std::make_unique<SomeType>(arg1, arg2, arg3); // 1
auto pPtr = std::make_unique<SomeType>(arg1, arg2, arg3); // 0
auto pPtr = std::make_unique(arg1, arg2, arg3); // 0
auto ptr = std::make_unique(arg1, arg2, arg3); // 1

auto uptr = std::make_shared<int>(); // 1
auto pUptr = std::make_shared<int>(); // 0
auto ptr = std::make_shared<int>(); // 1
auto uptr = std::make_shared<SomeType>(); // 1
auto pUptr = std::make_shared<SomeType>(); // 0
auto ptr = std::make_shared<SomeType>(); // 1
auto ptr = std::make_shared<SomeType>(arg1, arg2, arg3); // 1
auto pPtr = std::make_shared<SomeType>(arg1, arg2, arg3); // 0
auto pPtr = std::make_shared(arg1, arg2, arg3); // 0
auto ptr = std::make_shared(arg1, arg2, arg3); // 1

std::unique_ptr<int> ptr = foo(); // 1
std::unique_ptr<int> ptr = std::make_unique(); // 1

std::weak_ptr<int> ptr = foo(); // 1
std::weak_ptr<int> ptr = std::make_shared(); // 1
std::weak_ptr<int> pPtr = foo(); // 0
std::weak_ptr<int> pPtr = std::make_shared(); // 0

int *i = new int; // 1
int *pI = new int; // 0
int *i = foo(); // 1
int *pI = foo(); // 0
auto *i = new int; // 1
auto *pI = new int; // 0
auto *i = new QDialog(arg1, arg2, arg3); // 1
auto *pI = new QDialog(arg1, arg2, arg3); // 0
