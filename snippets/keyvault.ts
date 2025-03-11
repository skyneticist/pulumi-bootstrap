
const keyVault = new azure.keyvault.Vault("myKeyVault", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    properties: {
        sku: { name: "standard", family: "A" },
        tenantId: "YOUR_TENANT_ID", // Replace with your Azure tenant ID
        accessPolicies: [], // Add access policies here
    },
});

export const keyVaultUri = keyVault.properties?.vaultUri;
