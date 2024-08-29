using System.Data;
using Dapper;
using MySqlConnector;

namespace MasjidApp.API.Library.Shared.DataAccess;


internal sealed class DataAccess<TDbProvider> : IDataAccess  
    where TDbProvider : IDbConnection, new()
{
    private readonly TDbProvider _connection;
    
    public DataAccess(string connectionString)
    {
        _connection = new TDbProvider
        {
            ConnectionString = connectionString
        };
    }

    public async Task<int> ReadRecordCountFromDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters)
    {
        int count = await _connection.ExecuteScalarAsync<int>(storedProcedure, parameters, commandType: CommandType.StoredProcedure); 
        return count;
    }
    
    /// <summary>
    /// Returns a collection of data from an SQL query.
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    public async Task<IEnumerable<TClass>> ReadRecordsFromDatabaseAsync<TClass>(string storedProcedure)
    {
        IEnumerable<TClass> records = await _connection.QueryAsync<TClass>(storedProcedure, commandType: CommandType.StoredProcedure);
        return records;
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
        IEnumerable<TClass> records = await _connection.QueryAsync<TClass>(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
        return records;
    }
    
    
    /// <summary>
    /// Returns a single record from a SQL Query
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    public async Task<TClass> ReadFirstRecordFromDatabaseAsync<TClass>(string storedProcedure)
    {
        TClass record = await _connection.QueryFirstOrDefaultAsync(storedProcedure, commandType: CommandType.StoredProcedure);
        return record;
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
        TClass record = await _connection.QueryFirstOrDefaultAsync(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
        return record;
    }

    
    public async Task WriteToDatabaseAsync<TParameters> (string storedProcedure, TParameters parameters)
    {
        await _connection.ExecuteAsync(storedProcedure, parameters, commandType: CommandType.StoredProcedure);
    }
    
    /// <summary>
    /// Closes any connection
    /// </summary>
    public void Dispose()
    {
        _connection.Close();
    }
}