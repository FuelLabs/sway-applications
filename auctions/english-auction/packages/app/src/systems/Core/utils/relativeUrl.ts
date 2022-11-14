import { urlJoin } from 'url-join-ts';

export function relativeUrl(path: string) {
  return urlJoin(window.location.origin, process.env.PUBLIC_URL, path);
}
