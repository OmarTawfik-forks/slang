// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangBindings {
  export { BindingGraph };
  export { Definition };
  export { Reference };
  export { UserFileLocation };
  export { BuiltInLocation };
  export { BindingLocation };
  export { BindingLocationVariant };
}
import type { Cursor } from "./nomic-foundation-slang-cst.js";
export { Cursor };
/**
 * Represents a location of a symbol (definition or reference) in the binding graph.
 * It can either be in a user file, or a built-in in the language.
 */
export type BindingLocation = UserFileLocation | BuiltInLocation;
export enum BindingLocationVariant {
  UserFileLocation = "UserFileLocation",
  BuiltInLocation = "BuiltInLocation",
}

export class BindingGraph {
  /**
   * If the provided cursor points at a definition `Identifier`, it will return the
   * corresponding definition. Otherwise, it will return `undefined`.
   */
  definitionAt(cursor: Cursor): Definition | undefined;
  /**
   * If the provided cursor points at a reference `Identifier`, it will return the
   * corresponding reference. Otherwise, it will return `undefined`.
   */
  referenceAt(cursor: Cursor): Reference | undefined;
}

export class BuiltInLocation {
  readonly bindingLocationVariant = BindingLocationVariant.BuiltInLocation;

  asBuiltInLocation(): this;
  isBuiltInLocation(): this is BuiltInLocation;

  asUserFileLocation(): undefined;
  isUserFileLocation(): false;
}

export class Definition {
  /**
   * Returns a unique numerical identifier of the definition.
   * It is only valid for the lifetime of the binding graph.
   * It can change between multiple graphs, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the location of the definition's name.
   * For `contract X {}`, that is the location of the `X` `Identifier` node.
   */
  get nameLocation(): BindingLocation;
  /**
   * Returns the location of the definition's definiens.
   * For `contract X {}`, that is the location of the parent `ContractDefinition` node.
   */
  get definiensLocation(): BindingLocation;
}

export class Reference {
  /**
   * Returns a unique numerical identifier of the reference.
   * It is only valid for the lifetime of the binding graph.
   * It can change between multiple graphs, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the location of the reference.
   * For `new X()`, that is the location of the `X` `Identifier` node.
   */
  get location(): BindingLocation;
  /**
   * Returns a list of all definitions related to this reference.
   * Most references have a single definition, but some have multiple, such as when a symbol
   * is imported from another file, and renamed (re-defined) in the current file.
   */
  definitions(): Definition[];
}

export class UserFileLocation {
  readonly bindingLocationVariant = BindingLocationVariant.UserFileLocation;

  asUserFileLocation(): this;
  isUserFileLocation(): this is UserFileLocation;

  asBuiltInLocation(): undefined;
  isBuiltInLocation(): false;

  /**
   * Returns the ID of the file that contains the symbol.
   */
  get fileId(): string;
  /**
   * Returns a cursor to the CST node that contains the symbol.
   */
  get cursor(): Cursor;
}
