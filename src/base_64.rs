mod consts;
use consts::GP_URLSAFE_BASE_64;

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    GP_URLSAFE_BASE_64.encode(token)
}

