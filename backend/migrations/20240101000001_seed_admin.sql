-- Seed Admin User (password is 'admin123')
-- Hash created with argon2
INSERT INTO users (name, email, password_hash, role) 
VALUES ('System Admin', 'admin@sjs.id', '$argon2id$v=19$m=4096,t=3,p=1$MmszdDlpN3cxcGoxMnR3eQ$Fm7YF7kHPrVnNfX2ZfVfVfVfVfVfVfVfVfVfVfVfVfU', 'ADMIN')
ON CONFLICT (email) DO NOTHING;
