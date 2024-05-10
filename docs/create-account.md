

## to reset passoword , we need to do
1. Create endpoint /generate-password-link that receive email
2. validate that email exists in Ldap and auth0
3. Return Success
4. Send email + token



## How to reset-password-form
1. Create /reset-password
2. Receive passwords + token
3. Validate token
4. Update ldap or auth0
5. Return sucess 



