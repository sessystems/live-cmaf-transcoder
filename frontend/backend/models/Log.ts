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

import { mapValues } from '../runtime';
import type { LogLevel } from './LogLevel';
import {
    LogLevelFromJSON,
    LogLevelFromJSONTyped,
    LogLevelToJSON,
    LogLevelToJSONTyped,
} from './LogLevel';

/**
 * 
 * @export
 * @interface Log
 */
export interface Log {
    /**
     * 
     * @type {LogLevel}
     * @memberof Log
     */
    level: LogLevel;
    /**
     * 
     * @type {string}
     * @memberof Log
     */
    text: string;
    /**
     * 
     * @type {number}
     * @memberof Log
     */
    timestamp: number;
}



/**
 * Check if a given object implements the Log interface.
 */
export function instanceOfLog(value: object): value is Log {
    if (!('level' in value) || value['level'] === undefined) return false;
    if (!('text' in value) || value['text'] === undefined) return false;
    if (!('timestamp' in value) || value['timestamp'] === undefined) return false;
    return true;
}

export function LogFromJSON(json: any): Log {
    return LogFromJSONTyped(json, false);
}

export function LogFromJSONTyped(json: any, ignoreDiscriminator: boolean): Log {
    if (json == null) {
        return json;
    }
    return {
        
        'level': LogLevelFromJSON(json['level']),
        'text': json['text'],
        'timestamp': json['timestamp'],
    };
}

  export function LogToJSON(json: any): Log {
      return LogToJSONTyped(json, false);
  }

  export function LogToJSONTyped(value?: Log | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'level': LogLevelToJSON(value['level']),
        'text': value['text'],
        'timestamp': value['timestamp'],
    };
}

