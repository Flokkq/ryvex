//! Trait impls for the Terminal Command api
use std::fmt::Display;

use crate::term::command::{
	cursor::{
		DisableBlinking,
		EnableBlinking,
		Hide,
		MoveDown,
		MoveLeft,
		MoveRight,
		MoveTo,
		MoveToColumn,
		MoveToNextLine,
		MoveToPreviousLine,
		MoveToRow,
		MoveUp,
		RestorePosition,
		SavePosition,
		SetCursorStyle,
		Show,
	},
	terminal::{
		Clear,
		Print,
		ScrollDown,
		ScrollUp,
		SetSize,
	},
	ExecuteApi,
};

impl ExecuteApi for MoveTo {}
impl ExecuteApi for MoveToNextLine {}
impl ExecuteApi for MoveToPreviousLine {}
impl ExecuteApi for MoveToColumn {}
impl ExecuteApi for MoveToRow {}
impl ExecuteApi for MoveUp {}
impl ExecuteApi for MoveDown {}
impl ExecuteApi for MoveLeft {}
impl ExecuteApi for MoveRight {}
impl ExecuteApi for SavePosition {}
impl ExecuteApi for RestorePosition {}
impl ExecuteApi for Hide {}
impl ExecuteApi for Show {}
impl ExecuteApi for EnableBlinking {}
impl ExecuteApi for DisableBlinking {}
impl ExecuteApi for SetCursorStyle {}

impl ExecuteApi for ScrollUp {}
impl ExecuteApi for ScrollDown {}
impl ExecuteApi for Clear {}
impl ExecuteApi for SetSize {}
impl<T: Display> ExecuteApi for Print<T> {}
