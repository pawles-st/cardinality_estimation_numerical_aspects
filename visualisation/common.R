read.data <- function(alg, prec, card, mult) {
	data.text <- paste(alg, prec, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
	filename <- paste0("../results/", data.text, ".txt")
	numbers <- scan(filename, what = double(), nmax = iters)
	return(numbers)
}
