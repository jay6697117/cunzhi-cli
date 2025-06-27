// 弹窗系统类型定义

export interface McpRequest {
  id: string
  message: string
  predefined_options?: string[]
  is_markdown?: boolean
}

export interface PopupState {
  visible: boolean
  loading: boolean
  submitting: boolean
  currentTheme: string
  request: McpRequest | null
}

// 新的结构化响应格式
export interface McpResponse {
  user_input: string | null
  selected_options: string[]
  images: ImageAttachment[]
  metadata: ResponseMetadata
}

export interface ImageAttachment {
  data: string
  media_type: string
  filename: string | null
}

export interface ResponseMetadata {
  timestamp: string | null
  request_id: string | null
  source: string | null
}

// 旧格式兼容性支持
export interface PopupResponse {
  type: 'text' | 'image' | 'option'
  text?: string
  options?: string[]
  images?: ImageData[]
}

export interface ImageData {
  type: 'image'
  source: {
    type: 'base64'
    media_type: string
    data: string
  }
}

// 事件类型
export type PopupEvent
  = | { type: 'show', payload: McpRequest }
    | { type: 'hide' }
    | { type: 'submit', payload: PopupResponse }
    | { type: 'cancel' }
    | { type: 'theme-change', payload: string }
    | { type: 'open-main-layout' }
