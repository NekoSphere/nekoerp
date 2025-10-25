pub const EMAIL_ERR: &str = "Email inválido";
pub const EMAIL_PATTERN: &str = r#"(?:[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+(?:\.[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9\x2d]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
pub const EMAIL_DEFAULT: &str = "johndoe@example.com";
pub const PASSWORD_ERR: &str = "Senha inválida: precisa ter entre 8 até 32 caracteres, maiúscula, minúscula, um número e um símbolo";
pub const PASSWORD_PATTERN: &str = r"^[^\s]{8,32}$";
pub const PASSWORD_DEFAULT: &str = "Exemplo@123";
pub const NAME_ERR: &str = "Nome inválido: use apenas letras (qualquer alfabeto), espaços, hífen, apóstrofo ou ponto; 2–100 caracteres";
pub const NAME_PATTERN: &str = r#"^[\p{L}][\p{L}\p{M}'\-\.\s]{0,98}[\p{L}]$"#;
pub const NAME_DEFAULT: &str = "João da Silva";
pub const AVATAR_ERR: &str = "Avatar inválido: precisa ser uma URL http(s) que termine em .webp";
pub const AVATAR_PATTERN: &str = r#"^https?://(?:[A-Za-z0-9\-._~%!$&'()*+,;=]+@)?(?:\[[0-9A-Fa-f:.]+\]|[A-Za-z0-9\-\.]+)(?::\d{1,5})?(?:/[^\s?#]*?\.webp)(?:\?[^\s#]*)?(?:#.*)?$"#;
pub const AVATAR_DEFAULT: &str = "https://example.com/avatar.webp";
pub const UUID_ERR: &str =
    "UUID inválido: precisa estar no formato 8-4-4-4-12 com caracteres hexadecimais";
pub const UUID_PATTERN: &str =
    r#"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"#;
