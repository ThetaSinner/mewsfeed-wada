export const TAG_SYMBOLS = {
  CASHTAG: "$",
  HASHTAG: "#",
  MENTION: "@",
  LINK: "^",
};

const MENTION_TAG_REGEX_STRING = `\\B\\${TAG_SYMBOLS.MENTION}\\S+`;
const MENTION_TAG_REGEX = new RegExp(MENTION_TAG_REGEX_STRING, "mi");

const LINK_TAG_REGEX_STRING = `\\B\\${TAG_SYMBOLS.LINK}\\S+`;
const LINK_TAG_REGEX = new RegExp(LINK_TAG_REGEX_STRING, "mi");

const regexpString = [
  `\\B\\${TAG_SYMBOLS.CASHTAG}\\w+`,
  `\\B\\${TAG_SYMBOLS.HASHTAG}\\w+`,
  MENTION_TAG_REGEX.source,
  `\\B\\${TAG_SYMBOLS.LINK}\\[[\\S ]+\\]`, // multi-word labeled url
  LINK_TAG_REGEX.source, // single-word labeled url
];

const TAG_REGEX = new RegExp(`${regexpString.join("|")}`, "mi");

const RAW_URL_REGEX =
  /(?:(?<!\^)\b(?:(?:[a-z][\w-]+:(?:\/{1,3}|[a-z0-9%])|[a-z0-9.-]+[.][a-z]{2,4}\/)(?:[^\s()<>]+|\(([^\s()<>]+|(\([^\s()<>]+\)))*\))+(?:\(([^\s()<>]+|(\([^\s()<>]+\)))*\)|[^\s`!()[\]{};:'".,<>?«»“”‘’])))/;

const TAG_OR_RAW_URL_REGEX = new RegExp(
  `(${[...regexpString, RAW_URL_REGEX.source].join("|")})`,
  "mi"
);

export const isTag = (text: string): boolean => TAG_REGEX.test(text);

export const isMentionTag = (text: string): boolean =>
  MENTION_TAG_REGEX.test(text);

export const isLinkTag = (text: string): boolean => LINK_TAG_REGEX.test(text);

export const isRawUrl = (text: string): boolean => RAW_URL_REGEX.test(text);

export const splitMewTextIntoParts = (text: string): string[] =>
  text
    .split(TAG_OR_RAW_URL_REGEX)
    .filter((part) => part !== undefined && part.length > 0);
