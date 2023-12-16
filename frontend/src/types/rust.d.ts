/* This file is generated and managed by tsync */

interface Todo {
  id: number;
  text: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateTodo {
  text: string;
}

interface UpdateTodo {
  text?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}
