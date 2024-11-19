import password_hasher

hashed = password_hasher.hash_password("my_secure_password")
print("Hashed password:", hashed)

is_valid = password_hasher.verify_password("my_secure_password", hashed)
print("Is valid:", is_valid)