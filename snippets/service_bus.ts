
import * as azure from "@pulumi/azure-native";

const serviceBusNamespace = new azure.servicebus.Namespace("myServiceBusNamespace", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    sku: {
        name: "Standard",
        tier: "Standard",
    },
});

const serviceBusQueue = new azure.servicebus.Queue("myQueue", {
    resourceGroupName: "myResourceGroup",
    namespaceName: serviceBusNamespace.name,
    enablePartitioning: true,
});

export const serviceBusNamespaceName = serviceBusNamespace.name;
export const serviceBusQueueName = serviceBusQueue.name;
