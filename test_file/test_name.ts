// @ts-ignore
import * as pulumi from "@pulumi/pulumi";
// @ts-ignore
import * as azure from "@pulumi/azure-native";

// Configuration variables (optional customization via Pulumi config)
const config = new pulumi.Config();
const location = config.get("location") || "eastus2"; // Default location
const resourceGroupName = config.get("resourceGroupName") || "test-rg";

// Create an Azure Resource Group
const resourceGroup = new azure.resources.ResourceGroup("example-rg", {
    resourceGroupName: resourceGroupName,
    location: location,
});

// Create an Azure Storage Account
const storageAccount = new azure.storage.StorageAccount("examplestorage", {
    resourceGroupName: resourceGroup.name,
    location: resourceGroup.location,
    sku: {
        name: "Standard_LRS", // Local-redundant storage
    },
    kind: "StorageV2", // General-purpose v2
});

// Export outputs
export const rgName = resourceGroup.name;
export const storageAccountName = storageAccount.name;