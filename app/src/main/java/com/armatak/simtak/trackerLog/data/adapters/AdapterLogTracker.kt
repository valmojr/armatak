package com.armatak.simtak.trackerLog.data.adapters

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.DiffUtil
import androidx.recyclerview.widget.ListAdapter
import com.armatak.simtak.R
import com.armatak.simtak.trackerLog.data.models.LogModel
import com.armatak.simtak.trackerLog.data.viewHolder.ViewHolderLogTracker

class AdapterLogTracker: ListAdapter<LogModel, ViewHolderLogTracker>(DIFF_CALLBACK) {
    companion object{
        private val DIFF_CALLBACK = object : DiffUtil.ItemCallback<LogModel>(){
            override fun areItemsTheSame(oldItem: LogModel, newItem: LogModel): Boolean {
                return oldItem.idLine == newItem.idLine
            }

            override fun areContentsTheSame(oldItem: LogModel, newItem: LogModel): Boolean {
                return oldItem == newItem
            }

        }
    }
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolderLogTracker {
        val layoutInflater = LayoutInflater.from(parent.context)
        return ViewHolderLogTracker(
            layoutInflater.inflate(
                R.layout.item_log_tracker,
                parent,
                false
            )
        )
    }

    override fun onBindViewHolder(holder: ViewHolderLogTracker, position: Int) {
        val item = getItem(position)
        holder.render(item)
    }
}