import { EkoDBClient } from '@ekodb-client/nodejs';

async function main() {
    // Initialize client
    const client = new EkoDBClient({
        baseURL: 'http://localhost:8080',
        apiKey: 'test-api-key'
    });

    // Define interface for type safety
    interface User {
        name: string;
        age: number;
    }

    // Create a test record
    const user = {
        name: { type: 'String' as const, value: 'Test User' },
        age: { type: 'Integer' as const, value: 25 }
    };

    try {
        // Insert record
        const id = await client.insert<User>('users', user);
        console.log('Inserted record with ID:', id);

        // Retrieve record
        const result = await client.findById<User>('users', id);
        console.log('Retrieved record:', result);
    } catch (error) {
        console.error('Error:', error);
    }
}

main().catch(console.error);