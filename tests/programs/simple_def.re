_(
	def(
		my_fn,
		x, y, z,
		print("x:", x, "y:", y, "z:", z)
	),
	my_fn(2, true, null),
	def(lambda, print(null)),
	lambda()
)