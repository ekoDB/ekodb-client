/**
 * Scripts Example - Using @ekodb/ekodb-client library
 * 
 * Demonstrates creating, managing, and executing scripts with the JavaScript client.
 * Covers: FindAll, Group, Sort, Limit, Count, and Script management operations.
 */

const { EkoDBClient, Stage, ChatMessage } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function setupTestData(client) {
  console.log('📋 Setting up test data...');
  
  for (let i = 1; i <= 10; i++) {
    const record = {
      name: `User ${i}`,
      age: 20 + i,
      status: i % 2 === 0 ? 'active' : 'inactive',
      score: i * 10,
    };
    
    await client.insert('users', record);
  }
  
  console.log('✅ Test data ready\n');
}

async function simpleQueryScript(client) {
  console.log('📝 Example 1: Simple Query Script\n');
  
  const script = {
    label: 'get_active_users',
    name: 'Get Active Users',
    description: 'Retrieve all active users',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('users')
    ],
    tags: ['users', 'query'],
  };
  
  // Save script
  const scriptId = await client.saveScript(script);
  console.log(`✅ Script saved: ${scriptId}`);
  
  // Call script (use label)
  const result = await client.callScript('get_active_users');
  console.log(`📊 Found ${result.records.length} records`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function parameterizedScript(client) {
  console.log('📝 Example 2: Parameterized Script\n');
  
  const script = {
    label: 'get_users_by_status',
    name: 'Get Users By Status',
    version: '1.0',
    parameters: {
      status: {
        required: false,
        default: 'active',
        description: 'Filter by user status',
      },
      limit: {
        required: false,
        default: 10,
        description: 'Maximum number of results',
      },
    },
    functions: [
      Stage.findAll('users')
    ],
    tags: ['users', 'parameterized'],
  };
  
  await client.saveScript(script);
  console.log('✅ Script saved');
  
  // Call with parameters
  const result = await client.callScript('get_users_by_status', {
    status: 'active',
    limit: 3
  });
  console.log(`📊 Found ${result.records.length} users (limited)`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
}

async function aggregationScript(client) {
  console.log('📝 Example 3: Aggregation Script\n');
  
  const script = {
    label: 'user_stats',
    name: 'User Statistics',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('users'),
      Stage.group(['status'], [
        {
          output_field: 'count',
          operation: 'Count',
        },
        {
          output_field: 'avg_score',
          operation: 'Average',
          input_field: 'score',
        },
      ]),
    ],
    tags: ['analytics'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('user_stats');
  console.log(`📊 Statistics: ${result.records.length} groups`);
  result.records.forEach((record) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function scriptManagement(client, getActiveUsersId, userStatsId) {
  console.log('📝 Example 4: Script Management\n');
  
  // List all scripts
  const scripts = await client.listScripts();
  console.log(`📋 Total scripts: ${scripts.length}`);
  
  // Get specific script (use encrypted ID)
  const script = await client.getScript(getActiveUsersId);
  console.log(`🔍 Retrieved script: ${script.name}`);
  
  // Update script (use encrypted ID)
  const updated = {
    label: 'get_active_users',
    name: 'Get Active Users (Updated)',
    description: 'Updated description',
    version: '1.1',
    parameters: {},
    functions: [Stage.findAll('users')],
    tags: ['users'],
  };
  await client.updateScript(getActiveUsersId, updated);
  console.log('✏️  Script updated');
  
  // Delete script (use ID) - handle error gracefully
  try {
    await client.deleteScript(userStatsId);
    console.log('🗑️  Script deleted');
  } catch (error) {
    console.log('ℹ️  Script delete skipped (may not exist)');
  }
  console.log();
  
  console.log('ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID');
  console.log('ℹ️  Only CALL can use either ID or label\n');
}

async function multiStageScript(client) {
  console.log('📝 Example 5: Multi-Stage Pipeline\n');
  
  const script = {
    label: 'top_users',
    name: 'Top Performing Users',
    version: '1.0',
    parameters: {
      min_score: {
        required: false,
        default: 50,
      }
    },
    functions: [
      Stage.findAll('users'),
      Stage.project(['name', 'score', 'status'], false),
    ],
    tags: ['analytics', 'reporting'],
  };
  
  await client.saveScript(script);
  console.log('✅ Multi-stage script saved');
  
  const result = await client.callScript('top_users', { min_score: 50 });
  console.log(`📊 Pipeline executed ${result.stats.stages_executed} stages`);
  console.log(`⏱️  Total execution time: ${result.stats.execution_time_ms}ms`);
  console.log('📈 Stage breakdown:');
  result.stats.stage_stats.forEach((stage, index) => {
    console.log(`   ${index + 1}. ${stage.stage}: ${stage.execution_time_ms}ms (${stage.input_count} → ${stage.output_count} records)`);
  });
  console.log();
}

async function countScript(client) {
  console.log('📝 Example 6: Count Users\n');
  
  const script = {
    label: 'count_users',
    name: 'Count All Users',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('users'),
      Stage.count(),
    ],
    tags: ['users', 'count'],
  };
  
  await client.saveScript(script);
  console.log('✅ Count script saved');
  
  const result = await client.callScript('count_users');
  console.log(`📊 Total user count: ${result.records[0]?.count?.value || result.records[0]?.count || 0}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
}

async function cleanup(client) {
  console.log('🧹 Cleaning up...');
  
  // Delete test collection
  await client.deleteCollection('users');
  console.log('✅ Deleted collection');
  
  // List and delete all test scripts
  const scripts = await client.listScripts();
  for (const script of scripts) {
    if (script.label.startsWith('get_') || script.label.startsWith('user_') || 
        script.label.startsWith('top_') || script.label.startsWith('create_')) {
      try {
        await client.deleteScript(script.id);
      } catch (error) {
        // Script might already be deleted
      }
    }
  }
  console.log('✅ Deleted test scripts\n');
}

async function main() {
  console.log('🚀 ekoDB Scripts Example (JavaScript Client)\n');
  
  try {
    // Create and initialize ekoDB client
    const client = new EkoDBClient(BASE_URL, API_KEY);
    await client.init();
    console.log('✅ Client initialized (token exchange automatic)\n');
    
    await setupTestData(client);
    const getActiveUsersId = await simpleQueryScript(client);
    await parameterizedScript(client);
    const userStatsId = await aggregationScript(client);
    await scriptManagement(client, getActiveUsersId, userStatsId);
    await multiStageScript(client);
    await countScript(client);
    await cleanup(client);
    
    console.log('✅ All examples completed successfully!');
    console.log('\n💡 Key Advantages of Using the Client:');
    console.log('   • Automatic token management');
    console.log('   • Type-safe Stage builders');
    console.log('   • ChatMessage helpers');
    console.log('   • Cleaner, more maintainable code');
    console.log('   • Built-in error handling');
  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
