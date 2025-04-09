import { Configuration, FFApi, type InConfigID } from "~/backend";
import type { FFConfig } from "~/backend/models/FFConfig";
import type { InFFCreate } from "~/backend/models/InFFCreate";
import type { InFFUpdate } from "~/backend/models/InFFUpdate";
import type { InSetState } from "~/backend/models/InSetState";
import type { OutGetAllFFConfig } from "~/backend/models/OutGetAllFFConfig";
import type { OutGetFFStatus } from "~/backend/models/OutGetFFStatus";
import type { State } from "~/backend/models/State";
import type { VideoAdaptationSet } from "~/backend/models/VideoAdaptationSet";
import type { VideoRepresentation } from "~/backend/models/VideoRepresentation";

export const useFFApi = () => {

    const configs = useFFConfigs();
    const status = useFFStatus();
    const runtimeConfig = useRuntimeConfig()
    const ffApi = new FFApi(new Configuration({basePath: runtimeConfig.public.apiBase}))

    async function create(serverUID: string, title: string, output: string): Promise<string> {
        const request: InFFCreate = {
            serverUid: serverUID,
            name: title,
            output: output
        };

        const uid: string = await ffApi.create({inFFCreate: request});

        await refresh_configs();
        await refresh_status();

        return uid;
    }

    async function remove(server_id: string, config_id: string): Promise<void> {
        const request: InConfigID = {
            configUid: config_id,
            serverUid: server_id,
        };

        await ffApi.remove({inConfigID: request});
        await refresh_configs();
        await refresh_status();
    }


    async function setState(server_uid: string, configUid: string, state: State) {
        const request: InSetState = {
            id: {
                serverUid: server_uid,
                configUid: configUid
            },
            state: state,
        };

        await ffApi.setConfigState({inSetState: request});
        await refresh_configs();
        await refresh_status();
    }

    async function update(config: FFConfig) {

        const request: InFFUpdate = {
            config: config,
        };

        await ffApi.update({inFFUpdate: request});
        await refresh_configs();
        await refresh_status();
    }

    async function refresh_configs() {
            const response: OutGetAllFFConfig = await ffApi.getAllConfigs();
            configs.value = response.configs;
    }

    async function getFFCmd(server_uid: string, configUid: string): Promise<string> {
        const request: InConfigID = {
            serverUid: server_uid,
            configUid: configUid
        };
        const response: string = await ffApi.getFfCommand({inConfigID: request});
        return response;
    }

    async function createDefaultVideoAdaptationSet() {
        const adaptationSet: VideoAdaptationSet = await ffApi.createDefaultVideoAdaptationSet();
        return adaptationSet;
    }

    async function createDefaultVideoRepresentation() {
        const representation: VideoRepresentation = await ffApi.createDefaultVideoRepresentation();
        return representation;
    }

    function getConfig(uid: string): FFConfig | undefined {
        return configs.value.find((config) => config.uid === uid);
    }

    async function refresh_status() {
    const response: OutGetFFStatus = await ffApi.getAllStatus();
        status.value = response.status;
    }

    return { create, remove, setState, update, refresh_configs, getFFCmd, createDefaultVideoAdaptationSet, createDefaultVideoRepresentation, getConfig, refresh_status };
}