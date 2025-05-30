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
/**
 * 
 * @export
 * @interface InFFCreate
 */
export interface InFFCreate {
    /**
     * 
     * @type {string}
     * @memberof InFFCreate
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof InFFCreate
     */
    output: string;
    /**
     * 
     * @type {string}
     * @memberof InFFCreate
     */
    serverUid: string;
}

/**
 * Check if a given object implements the InFFCreate interface.
 */
export function instanceOfInFFCreate(value: object): value is InFFCreate {
    if (!('name' in value) || value['name'] === undefined) return false;
    if (!('output' in value) || value['output'] === undefined) return false;
    if (!('serverUid' in value) || value['serverUid'] === undefined) return false;
    return true;
}

export function InFFCreateFromJSON(json: any): InFFCreate {
    return InFFCreateFromJSONTyped(json, false);
}

export function InFFCreateFromJSONTyped(json: any, ignoreDiscriminator: boolean): InFFCreate {
    if (json == null) {
        return json;
    }
    return {
        
        'name': json['name'],
        'output': json['output'],
        'serverUid': json['server_uid'],
    };
}

  export function InFFCreateToJSON(json: any): InFFCreate {
      return InFFCreateToJSONTyped(json, false);
  }

  export function InFFCreateToJSONTyped(value?: InFFCreate | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'name': value['name'],
        'output': value['output'],
        'server_uid': value['serverUid'],
    };
}

