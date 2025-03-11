
const redisCache = new azure.cache.Redis("myRedisCache", {
    location: "EastUS",
    resourceGroupName: "myResourceGroup",
    sku: {
        name: "Standard",
        family: "C",
        capacity: 1, // 1 = 1 GB cache size
    },
    enableNonSslPort: false,
    minimumTlsVersion: "1.2",
});

export const redisHostName = redisCache.hostname;
export const redisPrimaryKey = redisCache.primaryKey;
