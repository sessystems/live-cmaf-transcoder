import type { Server } from "~/backend";
import type { FFConfig } from "~/backend/models/FFConfig";


export const useUtils = () => {

  const ffconfigs = useFFConfigs();
  const servers = useServers();

  function formatBytes(usage: number): string {
    if (usage < 1024) {
      return `${usage} B`;
    }

    if (usage < 1024 * 1024) {
      return `${Math.round(100 * Number(usage) / 1024) / 100} KB`;
    }

    if (usage < 1024 * 1024 * 1024) {
      return `${Math.round(100 * Number(usage) / (1024 * 1024)) / 100} MB`;
    }

    return `${Math.round(100 * Number(usage) / (1024 * 1024 * 1024)) / 100} GB`;
  }

  function formatBitrate(usage: number): string {
    if (usage < 1000) {
      return `${usage}b/s`;
    }

    if (usage < 1000 * 1000) {
      return `${Math.round(100 * Number(usage) / 1000) / 100}kb/s`;
    }

    return `${Math.round(100 * Number(usage) / (1000 * 1000)) / 100}Mb/s`;
  }

  function toAbsoluteUrl(path: string) {

    if (path.startsWith('http')) {
      return path;
    }

    return `${window.location.origin}${path}`;
  }

  function get_dash_url(config: FFConfig): string {
    const server: Server|undefined  = servers.value.find(s => s.uid === config.serverUid);
    let url = server?.baseUrl ?? "http://localhost";
    if (url.endsWith("/")) {
      url = url.substring(0, url.length - 1);
    }

    if (config.output.startsWith("/")) {
      url += config.output;
    } else {
      url += "/" + config.output;
    }

    if (!url.endsWith("/")) {
      url += "/"
    }

    return url + "manifest.mpd";
  }

  function get_hls_url(config: FFConfig): string {
    const server: Server|undefined = servers.value.find(s => s.uid === config.serverUid);
    let url = server?.baseUrl ?? "http://localhost";
    if (url.endsWith("/")) {
      url = url.substring(0, url.length - 1);
    }
    
    if (config.output.startsWith("/")) {
      url += config.output;
    } else {
      url += "/" + config.output;
    }

    if (!url.endsWith("/")) {
      url += "/"
    }

    return url + "playlist.m3u8";
  }

  function check_output_path_valid(output: string, configserveruid: string, configuid: string): boolean {


    if (output.endsWith("/")) {
      output = output.substring(0, output.length - 1);
    }

    if (!output.startsWith("/")) {
      return false;
    }

    if (output === "/"
      || output === ""
      || output === "/api"
      || output === "/monitor"
      || output === "/config"
      || output === "/_nuxt"
      || output === "/index.html"
      || output === "/favicon.ico"
      || output === "/404.html"
      || output === "/200.html"
      || output.startsWith("/api/")
      || output.startsWith("/monitor/")
      || output.startsWith("/config/")
      || output.startsWith("/_nuxt/")
      || output.includes("..")
    ) {
      return false;
    }

    for (const c of ffconfigs.value) {

      if (configuid === c.uid) {
        continue;
      }

      if (configserveruid !== c.serverUid) {
        continue;
      }

      let co = c.output;
      if (co.endsWith("/")) {
        co = co.substring(0, co.length - 1);
      }

      if (co === output) {
        return false;
      }
    }

    return true
  }

  function copyToClipboard(text: string) {
    unsecuredCopyToClipboard(text);
    if (navigator.clipboard === undefined) {
      unsecuredCopyToClipboard(text);
    } else {
      navigator.clipboard.writeText(text)
    }
  }

  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement("textarea");
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.focus({ preventScroll: true })
    textArea.select();
    try {
      document.execCommand('copy');
    } catch (err) {
      console.error('Unable to copy to clipboard', err);
    }
    document.body.removeChild(textArea);
  }

  return { formatBytes, formatBitrate, toAbsoluteUrl, get_dash_url, get_hls_url, check_output_path_valid, copyToClipboard }
}