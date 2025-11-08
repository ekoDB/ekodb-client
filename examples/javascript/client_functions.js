/**
 * Saved Functions Example - Using @ekodb/ekodb-client library
 * 
 * This example demonstrates saved functions using the JavaScript client library.
 * Compare with http_functions.js to see the difference!
 */

const { EkoDBClient, Stage, ParameterValue, ChatMessage } = require('@ekodb/ekodb-client');
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

async function simpleQueryFunction(client) {
  console.log('📝 Example 1: Simple Query Function\n');
  
  const function1 = {
    label: 'get_active_users',
    name: 'Get Active Users',
    description: 'Retrieve all active users',
    version: '1.0',
    parameters: {},
    pipeline: [
      Stage.findAll('users')
    ],
    tags: ['users', 'query'],
  };
  
  // Save function
  const functionId = await client.saveFunction(function1);
  console.log(`✅ Function saved: ${functionId}`);
  
  // Call function (use label)
  const result = await client.callFunction('get_active_users');
  console.log(`📊 Found ${result.records.length} records`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return functionId;
}

async function parameterizedFunction(client) {
  console.log('📝 Example 2: Parameterized Function\n');
  
  const function2 = {
    label: 'get_users_by_status',
    name: 'Get Users By Status',
    version: '1.0',
    parameters: {
      status: {
        param_type: 'String',
        required: false,
        default: 'active',
        description: 'Filter by user status',
      },
      limit: {
        param_type: 'Integer',
        required: false,
        default: 10,
        description: 'Maximum number of results',
      },
    },
    pipeline: [
      Stage.findAll('users')
    ],
    tags: ['users', 'parameterized'],
  };
  
  await client.saveFunction(function2);
  console.log('✅ Function saved');
  
  // Call with parameters
  const result = await client.callFunction('get_users_by_status', {
    status: 'active',
    limit: 3
  });
  console.log(`📊 Found ${result.records.length} users (limited)`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
}

async function aggregationFunction(client) {
  console.log('📝 Example 3: Aggregation Function\n');
  
  const function3 = {
    label: 'user_stats',
    name: 'User Statistics',
    version: '1.0',
    parameters: {},
    pipeline: [
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
  
  await client.saveFunction(function3);
  console.log('✅ Function saved');
  
  const result = await client.callFunction('user_stats');
  console.log(`📊 Statistics: ${result.records.length} groups`);
  result.records.forEach((record) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return function3.label;
}

async function functionManagement(client, getActiveUsersId, userStatsLabel) {
  console.log('📝 Example 4: Function Management\n');
  
  // List all functions
  const functions = await client.listFunctions();
  console.log(`📋 Total functions: ${functions.length}`);
  
  // Get specific function (use encrypted ID)
  const func = await client.getFunction(getActiveUsersId);
  console.log(`🔍 Retrieved function: ${func.name}`);
  
  // Update function (use encrypted ID)
  const updated = {
    label: 'get_active_users',
    name: 'Get Active Users (Updated)',
    description: 'Updated description',
    version: '1.1',
    parameters: {},
    pipeline: [Stage.findAll('users')],
    tags: ['users'],
  };
  await client.updateFunction(getActiveUsersId, updated);
  console.log('✏️  Function updated');
  
  // Delete function (use label)
  await client.deleteFunction(userStatsLabel);
  console.log('🗑️  Function deleted\n');
  
  console.log('ℹ️  Note: GET/UPDATE/DELETE can use either ID or label');
  console.log('ℹ️  CALL can also use either ID or label\n');
}

async function pipelineStagesExample(client) {
  console.log('📝 Example 5: Complex Pipeline with Multiple Stages\n');
  
  const function5 = {
    label: 'top_users',
    name: 'Top Performing Users',
    version: '1.0',
    parameters: {
      min_score: {
        param_type: 'Integer',
        required: false,
        default: 50,
      }
    },
    pipeline: [
      Stage.findAll('users'),
      // Note: Query stage would filter but we'll demonstrate projection
      Stage.project(['name', 'score', 'status']),
      // Count the results
      Stage.count(),
    ],
    tags: ['analytics', 'reporting'],
  };
  
  await client.saveFunction(function5);
  console.log('✅ Multi-stage function saved');
  
  const result = await client.callFunction('top_users', { min_score: 50 });
  console.log(`📊 Pipeline executed ${result.stats.stages_executed} stages`);
  console.log(`⏱️  Total execution time: ${result.stats.execution_time_ms}ms`);
  console.log('📈 Stage breakdown:');
  result.stats.stage_stats.forEach((stage, index) => {
    console.log(`   ${index + 1}. ${stage.stage}: ${stage.execution_time_ms}ms (${stage.input_count} → ${stage.output_count} records)`);
  });
  console.log();
}

async function writeOperationsExample(client) {
  console.log('📝 Example 6: Write Operations in Functions\n');
  
  const function6 = {
    label: 'create_user',
    name: 'Create New User',
    version: '1.0',
    parameters: {
      name: {
        param_type: 'String',
        required: true,
        description: 'User name',
      },
      age: {
        param_type: 'Integer',
        required: true,
        description: 'User age',
      },
    },
    pipeline: [
      Stage.insert('users', {
        name: ParameterValue.parameter('name'),
        age: ParameterValue.parameter('age'),
        status: ParameterValue.literal('active'),
        score: ParameterValue.literal(0),
      }),
    ],
    tags: ['users', 'write'],
  };
  
  await client.saveFunction(function6);
  console.log('✅ Write function saved');
  
  const result = await client.callFunction('create_user', {
    name: 'New User',
    age: 25,
  });
  console.log(`✅ User created via function`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
}

async function cleanup(client) {
  console.log('🧹 Cleaning up...');
  
  // Delete test collection
  await client.deleteCollection('users');
  console.log('✅ Deleted collection');
  
  // List and delete all test functions
  const functions = await client.listFunctions();
  for (const func of functions) {
    if (func.label.startsWith('get_') || func.label.startsWith('user_') || 
        func.label.startsWith('top_') || func.label.startsWith('create_')) {
      try {
        await client.deleteFunction(func.label);
      } catch (error) {
        // Function might already be deleted
      }
    }
  }
  console.log('✅ Deleted test functions\n');
}

async function main() {
  console.log('🚀 ekoDB Saved Functions Example (JavaScript Client)\n');
  
  try {
    // Create and initialize ekoDB client
    const client = new EkoDBClient(BASE_URL, API_KEY);
    await client.init();
    console.log('✅ Client initialized (token exchange automatic)\n');
    
    await setupTestData(client);
    const getActiveUsersId = await simpleQueryFunction(client);
    await parameterizedFunction(client);
    const userStatsLabel = await aggregationFunction(client);
    await functionManagement(client, getActiveUsersId, userStatsLabel);
    await pipelineStagesExample(client);
    await writeOperationsExample(client);
    await cleanup(client);
    
    console.log('✅ All examples completed successfully!');
    console.log('\n💡 Key Advantages of Using the Client:');
    console.log('   • Automatic token management');
    console.log('   • Type-safe Stage builders');
    console.log('   • ParameterValue and ChatMessage helpers');
    console.log('   • Cleaner, more maintainable code');
    console.log('   • Built-in error handling');
  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
