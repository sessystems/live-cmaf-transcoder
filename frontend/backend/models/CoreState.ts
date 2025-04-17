/* tslint:disable */
/* eslint-disable */
/**
 * live-cmaf-transcoder
 * API for the Live CMAF Transcoder
 *
 * The version of the OpenAPI document: 0.1.51
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


/**
 * 
 * @export
 */
export const CoreState = {
    Stopped: 'Stopped',
    Waiting: 'Waiting',
    Running: 'Running',
    Error: 'Error'
} as const;
export type CoreState = typeof CoreState[keyof typeof CoreState];


export function instanceOfCoreState(value: any): boolean {
    for (const key in CoreState) {
        if (Object.prototype.hasOwnProperty.call(CoreState, key)) {
            if (CoreState[key as keyof typeof CoreState] === value) {
                return true;
            }
        }
    }
    return false;
}

export function CoreStateFromJSON(json: any): CoreState {
    return CoreStateFromJSONTyped(json, false);
}

export function CoreStateFromJSONTyped(json: any, ignoreDiscriminator: boolean): CoreState {
    return json as CoreState;
}

export function CoreStateToJSON(value?: CoreState | null): any {
    return value as any;
}

export function CoreStateToJSONTyped(value: any, ignoreDiscriminator: boolean): CoreState {
    return value as CoreState;
}

