# Set variables
$certSubject = "CN=DungeonCrawlerWorld"
$pfxPassword = "kelvincelciusfar"
$pfxPath = Join-Path -Path $PSScriptRoot -ChildPath "DungeonCrawlerWorld.pfx"
$encodedPath = Join-Path -Path $PSScriptRoot -ChildPath "DungeonCrawlerWorld_Encoded.txt"

# Create self-signed certificate
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject $certSubject -CertStoreLocation "Cert:\CurrentUser\My"

# Convert password to SecureString
$pfxPwd = ConvertTo-SecureString -String $pfxPassword -Force -AsPlainText

# Export the certificate as .pfx
Export-PfxCertificate -Cert $cert -FilePath $pfxPath -Password $pfxPwd

# Encode the .pfx as base64 and save to text file
$pfx_cert = Get-Content $pfxPath -Encoding Byte
[System.Convert]::ToBase64String($pfx_cert) | Out-File $encodedPath

Write-Host "PFX created at: $pfxPath"
Write-Host "Base64 encoded PFX saved at: $encodedPath"
Write-Host "Use the contents of $encodedPath for your GitHub secret 'Base64_Encoded_Pfx'."
Write-Host "Use the password '$pfxPassword' for your GitHub secret 'Pfx_Key'."
