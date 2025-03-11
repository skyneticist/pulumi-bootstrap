
const containerRegistry = new azure.containerregistry.Registry("myContainerRegistry", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    sku: {
        name: "Basic", // Options: Basic, Standard, Premium
    },
    adminUserEnabled: true,
});

export const registryName = containerRegistry.name;
export const registryLoginServer = containerRegistry.loginServer;
