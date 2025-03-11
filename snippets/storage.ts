// @ts-ignore
const storageAccount = new azure.storage.StorageAccount("myStorageAccount", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    sku: {
        name: "Standard_LRS",
    },
    kind: "StorageV2",
});

const storageContainer = new azure.storage.BlobContainer("myContainer", {
    resourceGroupName: "myResourceGroup",
    accountName: storageAccount.name,
    publicAccess: "Blob", // Allow public access to blobs
});

export const storageAccountName = storageAccount.name;
export const containerName = storageContainer.name;
