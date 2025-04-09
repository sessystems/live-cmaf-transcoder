import type { FFConfig } from "~/backend/models/FFConfig"
import type { FFStatusValue } from "~/backend/models/FFStatusValue"
import type { Server,ServerStatus } from "~/backend"

export const useTitle = () => useState<String>('title', () => '')
export const useCurrentConfigUid = () => useState<string | undefined>('currentConfigUid', () => undefined)
export const useServers = () => useState<Server[]>('servers', () => [])
export const useServerStatus = () => useState<ServerStatus[]>('serverStatus', () => [])

export const useFFConfigs = () => useState<FFConfig[]>('ffconfigs', () => [])
export const useFFStatus = () => useState<FFStatusValue[]>('ffstatus', () => [])

