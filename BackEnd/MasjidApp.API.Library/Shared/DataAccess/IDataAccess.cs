namespace MasjidApp.API.Library.Shared.DataAccess;

public interface IDataAccess : IDisposable
{
    Task<int> ReadRecordCountFromDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters);
    Task<IEnumerable<TClass>> ReadRecordsFromDatabaseAsync<TClass>(string storedProcedure);
    Task<IEnumerable<TClass>> ReadRecordsFromDatabaseWithParametersAsync<TClass, TParameters>(string storedProcedure, TParameters parameters);
    Task<TClass> ReadFirstRecordFromDatabaseAsync<TClass>(string storedProcedure);
    Task<TClass> ReadFirstRecordFromDatabaseWithParametersAsync<TClass, TParameters>(string storedProcedure, TParameters parameters);
    Task WriteToDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters);
}