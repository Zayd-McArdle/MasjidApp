namespace MasjidApp.API.Library.Shared.DataAccess;

public interface IDataAccess : IDisposable
{
    Task<int> ReadRecordCountFromDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters);

    /// <summary>
    /// Returns a collection of data from an SQL query.
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    Task<IEnumerable<TClass>> ReadRecordsFromDatabaseAsync<TClass>(string storedProcedure);

    /// <summary>
    /// Returns a collection of data from an SQL query.
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <typeparam name="TParameters">The parameter type; for example, dynamic, key value pair.</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    /// <param name="parameters">Parameters to be injected into the stored procedure</param>   
    Task<IEnumerable<TClass>> ReadRecordsFromDatabaseWithParametersAsync<TClass, TParameters>(string storedProcedure, TParameters parameters);

    /// <summary>
    /// Returns a single record from a SQL Query
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    Task<TClass> ReadFirstRecordFromDatabaseAsync<TClass>(string storedProcedure);

    /// <summary>
    /// Returns a single record from a SQL Query
    /// </summary>
    /// <typeparam name="TClass">The class the data will be mapped tp</typeparam>
    /// <typeparam name="TParameters">The parameter type; for example, dynamic, key value pair.</typeparam>
    /// <param name="storedProcedure">The stored procedure you want your SQL provider to execute</param>
    /// <param name="parameters">Parameters to be injected into the stored procedure</param>    
    Task<TClass> ReadFirstRecordFromDatabaseWithParametersAsync<TClass, TParameters>(string storedProcedure, TParameters parameters);

    Task WriteToDatabaseAsync<TParameters> (string storedProcedure, TParameters parameters);
}