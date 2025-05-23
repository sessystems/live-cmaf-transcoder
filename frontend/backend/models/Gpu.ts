/* tslint:disable */
/* eslint-disable */
/**
 * live-cmaf-transcoder
 * API for the Live CMAF Transcoder
 *
 * The version of the OpenAPI document: 0.1.63
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { Acceleration } from './Acceleration';
import {
    AccelerationFromJSON,
    AccelerationFromJSONTyped,
    AccelerationToJSON,
    AccelerationToJSONTyped,
} from './Acceleration';

/**
 * 
 * @export
 * @interface Gpu
 */
export interface Gpu {
    /**
     * 
     * @type {Acceleration}
     * @memberof Gpu
     */
    acceleration: Acceleration;
    /**
     * 
     * @type {number}
     * @memberof Gpu
     */
    index: number;
    /**
     * 
     * @type {string}
     * @memberof Gpu
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof Gpu
     */
    uid: string;
}



/**
 * Check if a given object implements the Gpu interface.
 */
export function instanceOfGpu(value: object): value is Gpu {
    if (!('acceleration' in value) || value['acceleration'] === undefined) return false;
    if (!('index' in value) || value['index'] === undefined) return false;
    if (!('name' in value) || value['name'] === undefined) return false;
    if (!('uid' in value) || value['uid'] === undefined) return false;
    return true;
}

export function GpuFromJSON(json: any): Gpu {
    return GpuFromJSONTyped(json, false);
}

export function GpuFromJSONTyped(json: any, ignoreDiscriminator: boolean): Gpu {
    if (json == null) {
        return json;
    }
    return {
        
        'acceleration': AccelerationFromJSON(json['acceleration']),
        'index': json['index'],
        'name': json['name'],
        'uid': json['uid'],
    };
}

  export function GpuToJSON(json: any): Gpu {
      return GpuToJSONTyped(json, false);
  }

  export function GpuToJSONTyped(value?: Gpu | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'acceleration': AccelerationToJSON(value['acceleration']),
        'index': value['index'],
        'name': value['name'],
        'uid': value['uid'],
    };
}

