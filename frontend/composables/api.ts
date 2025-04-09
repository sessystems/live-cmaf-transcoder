import { Configuration, ServersApi, type GetLogsRequest, type Log, type OutGetAllServers, type OutGetAllServerStatus, type OutGetLogs, type RemoveServerRequest } from "~/backend";

export const useApi = () => {

  const servers = useServers();
  const serverStatus = useServerStatus();
  const runtimeConfig = useRuntimeConfig()
  const serverApi = new ServersApi(new Configuration({basePath: runtimeConfig.public.apiBase}));

  async function refresh_servers() {
    const response: OutGetAllServers = await serverApi.getAllServers();
    response.servers.sort((a, b) => a.name.localeCompare(b.name));
    servers.value = response.servers;
  }

  async function refreshServerStatus() {
    const response: OutGetAllServerStatus = await serverApi.getAllServerStatus();
    serverStatus.value = response.status;
  }

  async function getLogs(server_uid: string, configUid: string): Promise<Array<Log>> {
    const request: GetLogsRequest = {
      inConfigID: {configUid: configUid, serverUid: server_uid},
    };
    
    const response: OutGetLogs = await serverApi.getLogs(request);

    return response.logs;
  }


  async function removeServer(server_uid: string) {
    const request: RemoveServerRequest = {
      inServerID: {serverUid: server_uid},
    };

    await serverApi.removeServer(request);
    await refresh_servers();
  }

  return {
    refresh_servers, refreshServerStatus,
    removeServer,
    getLogs
  };
};
