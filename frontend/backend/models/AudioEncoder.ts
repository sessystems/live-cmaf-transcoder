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
export const AudioEncoder = {
    Aac: 'Aac',
    FdkAac: 'FDKAac'
} as const;
export type AudioEncoder = typeof AudioEncoder[keyof typeof AudioEncoder];


export function instanceOfAudioEncoder(value: any): boolean {
    for (const key in AudioEncoder) {
        if (Object.prototype.hasOwnProperty.call(AudioEncoder, key)) {
            if (AudioEncoder[key as keyof typeof AudioEncoder] === value) {
                return true;
            }
        }
    }
    return false;
}

export function AudioEncoderFromJSON(json: any): AudioEncoder {
    return AudioEncoderFromJSONTyped(json, false);
}

export function AudioEncoderFromJSONTyped(json: any, ignoreDiscriminator: boolean): AudioEncoder {
    return json as AudioEncoder;
}

export function AudioEncoderToJSON(value?: AudioEncoder | null): any {
    return value as any;
}

export function AudioEncoderToJSONTyped(value: any, ignoreDiscriminator: boolean): AudioEncoder {
    return value as AudioEncoder;
}

