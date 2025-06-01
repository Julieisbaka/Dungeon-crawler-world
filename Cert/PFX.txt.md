```powershell
# Generate a self-signed certificate and export as .pfx
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=YourName" -CertStoreLocation "Cert:\CurrentUser\My"
$pwd = ConvertTo-SecureString -String "your_password" -Force -AsPlainText
Export-PfxCertificate -Cert $cert -FilePath ".\SigningCertificate.pfx" -Password $pwd

```

```powershell
$pfx_cert = Get-Content '.\SigningCertificate.pfx' -Encoding Byte
[System.Convert]::ToBase64String($pfx_cert) | Out-File 'SigningCertificate_Encoded.txt'
```
