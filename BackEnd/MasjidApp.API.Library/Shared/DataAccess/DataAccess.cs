using System.Data;
using Dapper;
using MySqlConnector;

namespace MasjidApp.API.Library.Shared.DataAccess;


internal sealed class DataAccess<TDbProvider> : IDataAccess, IDisposable  
    where TDbProvider : IDbConnection, new()
{
    private readonly TDbProvider _connection;
    private readonly string _connectionString;
    private readonly bool _isPersistentConnection;

    private TDbProvider CreateDbConnection()
    {
        TDbProvider connection = new TDbProvider
        {
            ConnectionString = _connectionString
        };
        _connection.Open();
        return connection;
    }
    public DataAccess(string connectionString, bool isPersistentConnection = false)
    {
        _connectionString = connectionString;
        _isPersistentConnection = isPersistentConnection;
        if (_isPersistentConnection)
        {
            _connection = CreateDbConnection();
        }
    }

    public async Task<int> ReadRecordCountFromDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters)
    {
        if (_isPersistentConnection)
        {
            int count = await _connection.ExecuteScalarAsync<int>(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
            return count;
        }

        using IDbConnection connection = CreateDbConnection();
        {
            int count = await connection.ExecuteScalarAsync<int>(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
            return count;
        }
    }
    
    /// <summary>
    /// Returns a collection of data from an SQL query.
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    public async Task<IEnumerable<TClass>> ReadRecordsFromDatabaseAsync<TClass>(string storedProcedure)
    {
        if (_isPersistentConnection)
        {
            IEnumerable<TClass> records =
                await _connection.QueryAsync<TClass>(storedProcedure, commandType: CommandType.StoredProcedure);
            return records;
        }

        using TDbProvider connection = CreateDbConnection();
        {
            IEnumerable<TClass> records = await connection.QueryAsync<TClass>(storedProcedure, commandType: CommandType.StoredProcedure);
            return records;
        }
    }
    
    /// <summary>
    /// Returns a collection of data from an SQL query.
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <typeparam name="TParameters">The parameter type; for example, dynamic, key value pair.</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    /// <param name="parameters">Parameters to be injected into the stored procedure</param>   
    public async Task<IEnumerable<TClass>> ReadRecordsFromDatabaseWithParametersAsync<TClass, TParameters>(string storedProcedure, TParameters parameters)
    {
        if (_isPersistentConnection)
        {
            IEnumerable<TClass> records =
                await _connection.QueryAsync<TClass>(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
            return records;
        }

        using TDbProvider connection = CreateDbConnection();
        {
            IEnumerable<TClass> records = await connection.QueryAsync<TClass>(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
            return records;
        }
    }
    
    
    /// <summary>
    /// Returns a single record from a SQL Query
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    public async Task<TClass> ReadFirstRecordFromDatabaseAsync<TClass>(string storedProcedure)
    {
        if (_isPersistentConnection)
        {
            TClass record =
                await _connection.QueryFirstOrDefaultAsync(storedProcedure, commandType: CommandType.StoredProcedure);
            return record;
        }

        using TDbProvider connection = CreateDbConnection();
        {
            TClass record = await connection.QueryFirstOrDefaultAsync<TClass>(storedProcedure, commandType: CommandType.StoredProcedure);
            return record;
        }
    }

    /// <summary>
    /// Returns a single record from a SQL Query
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <typeparam name="TParameters">The parameter type; for example, dynamic, key value pair.</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    /// <param name="parameters">Parameters to be injected into the stored procedure</param>    
    public async Task<TClass> ReadFirstRecordFromDatabaseWithParametersAsync<TClass, TParameters>(string storedProcedure, TParameters parameters)
    {
        if (_isPersistentConnection)
        {
            TClass record =
                await _connection.QueryFirstOrDefaultAsync(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
            return record;
        }

        using TDbProvider connection = CreateDbConnection();
        {
            TClass record = await connection.QueryFirstOrDefaultAsync<TClass>(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
            return record;
        }
    }

    
    public async Task WriteToDatabaseAsync<TParameters> (string storedProcedure, TParameters parameters)
    {
        if (_isPersistentConnection)
        {
            await _connection.ExecuteAsync(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
        }

        using TDbProvider connection = CreateDbConnection();
        await connection.ExecuteAsync(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
    }
    
    /// <summary>
    /// Closes any connection
    /// </summary>
    public void Dispose()
    {
        _connection.Close();
    }
}