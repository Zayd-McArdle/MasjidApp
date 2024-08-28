namespace MasjidApp.API.Library.Shared.DataAccess;

internal interface IDataAccess
{
    Task<int> ReadRecordCountFromDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters);
    Task<IEnumerable<TClass>> ReadRecordsFromDatabaseAsync<TClass, TParameters>(string storedProcedure, TParameters parameters);
    Task<TClass> ReadFirstRecordFromDatabaseAsync<TClass, TParameters>(string storedProcedure, TParameters parameters);
    Task WriteToDatabaseAsync<TParameters>(string storedProcedure, TParameters parameters);
}